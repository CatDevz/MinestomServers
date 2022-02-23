use crate::database::queries::server::ServerQuery;
use crate::models::server::Server;
use crate::database::PgPool;
use crate::routes::{ApiError, db_conn_from_pool_or_err};

use actix_web::{HttpResponse, Responder, Result, get, post, web};

#[get("/servers/{id}")]
pub async fn get_server_by_id(
    id: web::Path<String>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, ApiError> {
    let connection = db_conn_from_pool_or_err(&pool)?;
    let query = ServerQuery::get_by_id(connection, &id)?;

    if let Some(data) = query {
        let response = Server::from(data);
        Ok(HttpResponse::Ok().json(response))
    } else {
        Ok(HttpResponse::NotFound().body("Not found"))
    }
}
