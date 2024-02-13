#![allow(clippy::needless_lifetimes)]
#![allow(clippy::needless_return)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use quote::quote_api_client::QuoteApiClient;
pub mod quote {
    tonic::include_proto!("quote");
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port: String = std::env::var("SERVER_PORT").unwrap_or_else(|_| String::from("50051"));

    let host: String = std::env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1"));

    let protocol_scheme: String =
        std::env::var("PROTOCOL_SCHEME").unwrap_or_else(|_| String::from("http"));

    let address: String = format!("{protocol_scheme}://{host}:{port}");

    // let mut client = QuoteApiClient::connect(address).await?;

    let response = QuoteApiClient::connect(address)
        .await?
        .read(quote::QuoteFilter {})
        .await?;

    println!("RESPONSE={response:?}");

    Ok(())
}
