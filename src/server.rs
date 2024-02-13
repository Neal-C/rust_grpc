#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::needless_lifetimes)]

use chrono::Timelike;
use sqlx::postgres::PgPoolOptions;
use tonic::Code;
use tonic::{transport::Server, Request, Response, Status};
mod quote_repository;
use quote_repository::Repository as QuoteRepository;
mod quote_model;
use crate::quote_model::AppQuoteUpdateRequest;
use quote_model::AppQuote;
use std::str::FromStr;
mod quote {
    tonic::include_proto!("quote"); // The string specified here must match the proto package name
    pub const QUOTE_DESCRIPTIOR_FOR_GRPC_REFLECTION: &[u8] =
        tonic::include_file_descriptor_set!("quote_descriptor");
}

use quote::quote_api_server::{QuoteApi, QuoteApiServer};
use quote::{
    Quote, QuoteCreateRequest, QuoteEmptyOkReponse, QuoteFilter, QuoteList, QuoteReadOneRequest, QuoteRemoveRequest, QuoteUpdateRequest
};

impl From<AppQuote> for Quote {
    fn from(payload: AppQuote) -> Self {
        let created_at_grpc_timestamptz = prost_types::Timestamp {
            seconds: payload.created_at.second().into(),
            ..Default::default()
        };
        let updated_at_grpc_timestamptz = prost_types::Timestamp {
            seconds: payload.updated_at.second().into(),
            ..Default::default()
        };

        Self {
            id: payload.id.to_string(),
            book: payload.book,
            quote: payload.quote,
            created_at: Some(created_at_grpc_timestamptz),
            updated_at: Some(updated_at_grpc_timestamptz),
        }
    }
}

#[derive(Debug)]
pub struct MyQuoteApi {
    quote_repository: QuoteRepository,
}

impl MyQuoteApi {
    const fn new(quote_repository: QuoteRepository) -> Self {
        Self { quote_repository }
    }
}

#[tonic::async_trait]
impl QuoteApi for MyQuoteApi {
    async fn create(
        &self,
        request: Request<QuoteCreateRequest>,
    ) -> Result<Response<Quote>, Status> {
        let request = request.into_inner();

        let app_quote: AppQuote = AppQuote::from(request);

        let Ok(_) = self.quote_repository.insert(app_quote.clone()).await else {
            return Err(Status::new(
                Code::Internal,
                "whoops. Please punch your developer or closest nerd",
            ));
        };

        let reply = Quote::from(app_quote);

        Ok(Response::new(reply))
    }

    async fn read(&self, request: Request<QuoteFilter>) -> Result<Response<QuoteList>, Status> {
        println!("Incoming request : {request:?}");

        let Ok(quotes) = self.quote_repository.find_all().await else {
            return Err(Status::not_found("no quote found"));
        };

        let quotes = quotes.into_iter().map(Quote::from).collect::<Vec<Quote>>();

        let quote_list = QuoteList { data: quotes };

        Ok(Response::new(quote_list))
    }

    async fn read_one(
        &self,
        request: Request<QuoteReadOneRequest>,
    ) -> Result<Response<Quote>, Status> {
        println!("Incoming request : {request:?}");

        let Ok(id) = uuid::Uuid::from_str(&request.into_inner().id) else {
            return Err(Status::invalid_argument("id was not a uuid"));
        };

        let Ok(quote) = self.quote_repository.find_by_id(id).await else {
            return Err(Status::not_found("nothing found"));
        };
        let reply = Quote::from(quote);

        Ok(Response::new(reply))
    }

    async fn update(
        &self,
        request: Request<QuoteUpdateRequest>,
    ) -> Result<Response<Quote>, Status> {
        println!("Incoming request : {request:?}");

        let request = request.into_inner();

        let Ok(payload) = AppQuoteUpdateRequest::try_from(request) else {
            return Err(Status::invalid_argument(
                "invalid request, probably a misformed id",
            ));
        };

        let Ok(result) = self.quote_repository.update(payload).await else {
            return Err(Status::not_found("no quote found"));
        };

        let reply = Quote::from(result);

        Ok(Response::new(reply))
    }

    async fn delete(
        &self,
        request: Request<QuoteRemoveRequest>,
    ) -> Result<Response<QuoteEmptyOkReponse>, Status> {
        println!("Incoming request : {request:?}");

        let Ok(id) = uuid::Uuid::from_str(&request.into_inner().id) else {
            return Err(Status::invalid_argument("id was not a uuid"));
        };

        let Ok(_pg_row) = self.quote_repository.delete_by_id(id).await else {
            return Err(Status::not_found("nothing found"));
        };

        Ok(Response::new(QuoteEmptyOkReponse{}))
    }
}

const MAX_PG_CONNECTIONS: u32 = 5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url: String =
        std::env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable");

    let port: String = std::env::var("SERVER_PORT").unwrap_or_else(|_| String::from("50051"));

    let host: String = std::env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1"));

    let address: String = format!("{host}:{port}");

    let address: std::net::SocketAddr = address.parse()?;

    let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(MAX_PG_CONNECTIONS)
        .connect(&database_url)
        .await?;

    let quote_repository: QuoteRepository = QuoteRepository::new(pool);

    let my_quote_api: MyQuoteApi = MyQuoteApi::new(quote_repository);

    let reflection_server = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(quote::QUOTE_DESCRIPTIOR_FOR_GRPC_REFLECTION)
        .build()?;

    Server::builder()
        // GrpcWeb is over http1 so we must enable it.
        .accept_http1(true)
        .add_service(QuoteApiServer::new(my_quote_api))
        .add_service(reflection_server)
        .serve(address)
        .await?;

    Ok(())
}
