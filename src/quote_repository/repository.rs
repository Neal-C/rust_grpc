#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::needless_return)]

use sqlx::{Pool, Postgres};

use crate::quote_model::{AppQuote, AppQuoteUpdateRequest};

#[derive(Debug)]
pub struct Repository {
    connection_pool: Pool<Postgres>,
}

impl Repository {
    pub const fn new(connection_pool: Pool<Postgres>) -> Self {
        Self { connection_pool }
    }

    pub async fn insert(
        &self,
        quote: AppQuote,
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        let postgres_query_result: Result<sqlx::postgres::PgQueryResult, sqlx::Error> = sqlx::query(
            "INSERT INTO quote (id, book, quote, created_at, updated_at) VALUES ($1,$2,$3,$4,$5)",
        )
        .bind(quote.id)
        .bind(&quote.book)
        .bind(&quote.quote)
        .bind(quote.created_at)
        .bind(quote.updated_at)
        .execute(&self.connection_pool)
        .await;

        match postgres_query_result {
            Ok(pg_query_result) => return Ok(pg_query_result),
            Err(sqlx::Error::RowNotFound) => return Err(sqlx::Error::RowNotFound),
            Err(mysterious_error) => {
                println!("MYSTERIOUS ERROR ----- LOGGED");
                println!("{mysterious_error}");
                return Err(mysterious_error);
            }
        }
    }

    pub async fn find_all(&self) -> Result<Vec<AppQuote>, sqlx::Error> {
        let postgres_query_result: Result<Vec<AppQuote>, sqlx::Error> =
            sqlx::query_as::<_, AppQuote>(
                "SELECT id, book, quote, created_at, updated_at FROM quote",
            )
            .fetch_all(&self.connection_pool)
            .await;

        match postgres_query_result {
            Ok(quotes) => return Ok(quotes),
            Err(sqlx::Error::RowNotFound) => return Err(sqlx::Error::RowNotFound),
            Err(mysterious_error) => {
                println!("MYSTERIOUS ERROR ----- LOGGED");
                println!("{mysterious_error}");
                return Err(mysterious_error);
            }
        }
    }

    pub async fn find_by_id(&self, id: uuid::Uuid) -> Result<AppQuote, sqlx::Error> {
        let postgres_query_result = sqlx::query_as::<_, AppQuote>(
            "SELECT id, book, quote, created_at, updated_at FROM quote WHERE id = $1",
        )
        .bind(id)
        .fetch_one(&self.connection_pool)
        .await;

        println!("RESULT: {postgres_query_result:?}");

        match postgres_query_result {
            Ok(quote) => return Ok(quote),
            Err(sqlx::Error::RowNotFound) => return Err(sqlx::Error::RowNotFound),
            Err(mysterious_error) => {
                println!("MYSTERIOUS ERROR ----- LOGGED");
                println!("{mysterious_error}");
                return Err(mysterious_error);
            }
        }
    }

    pub async fn update(&self, payload: AppQuoteUpdateRequest) -> Result<AppQuote, sqlx::Error> {
        let updated_at_timestamptz = chrono::Utc::now();

        let postgres_query_result: Result<AppQuote, sqlx::Error> = sqlx::query_as::<_, AppQuote>(
            "UPDATE quote SET (quote, updated_at) = ($2,$3) WHERE quote.id = $1 RETURNING *;",
        )
        .bind(payload.id)
        .bind(&payload.quote)
        .bind(updated_at_timestamptz)
        .fetch_one(&self.connection_pool)
        .await;

        match postgres_query_result {
            Ok(updated_quote) => return Ok(updated_quote),
            Err(sqlx::Error::RowNotFound) => return Err(sqlx::Error::RowNotFound),
            Err(sqlx::Error::Database(database_error)) => {
                println!("DATABASE ERROR ----- LOGGED");
                println!("{database_error}");
                return Err(sqlx::Error::Database(database_error));
            }
            Err(mysterious_error) => {
                println!("MYSTERIOUS ERROR ----- LOGGED");
                println!("{mysterious_error}");
                return Err(mysterious_error);
            }
        }
    }

    pub async fn delete_by_id(
        &self,
        id: uuid::Uuid,
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        let postgres_query_result = sqlx::query(
            "
            DELETE FROM quote WHERE quote.id = $1
            ",
        )
        .bind(id)
        .execute(&self.connection_pool)
        .await;

        match postgres_query_result {
            Ok(pg_row) => match pg_row.rows_affected() {
                0 => return Err(sqlx::Error::RowNotFound),
                _ => Ok(pg_row)
            },
            Err(sqlx::Error::RowNotFound) => return Err(sqlx::Error::RowNotFound),
            Err(sqlx::Error::Database(database_error)) => {
                println!("DATABASE ERROR ----- LOGGED");
                println!("{database_error}");
                return Err(sqlx::Error::Database(database_error));
            }
            Err(mysterious_error) => {
                println!("MYSTERIOUS ERROR ----- LOGGED");
                println!("{mysterious_error}");
                return Err(mysterious_error);
            }
        }
    }
}
