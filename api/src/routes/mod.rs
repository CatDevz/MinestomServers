use crate::database::{PgPool, PgPooledConnection};

use std::fmt;

pub fn db_conn_from_pool_or_err(pool: &PgPool) -> Result<PgPooledConnection, ApiError> {
    match pool.get() {
        Ok(connection) => Ok(connection),
        Err(_) => return Err(ApiError::DatabaseError),
    }
}

#[derive(Debug)]
pub enum ApiError {
    DatabaseError,
}

impl fmt::Display for ApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::DatabaseError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::web::HttpResponse {
        actix_web::web::HttpResponse::build(self.status_code()).json(
            crate::models::error::ApiError {
                error: match self {
                    ApiError::DatabaseError => String::from("database_error"),
                },
                description: match self {
                    ApiError::DatabaseError => String::from("A database error occured"),
                }
            }
        )
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(_: diesel::result::Error) -> Self { 
        ApiError::DatabaseError
    }
}

pub mod server;
