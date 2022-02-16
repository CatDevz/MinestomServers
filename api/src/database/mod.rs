use diesel::r2d2::{Pool, ConnectionManager, PooledConnection};
use diesel::PgConnection;

pub type PgConnectionManager = ConnectionManager<PgConnection>;
pub type PgPool = Pool<PgConnectionManager>;
pub type PgPooledConnection = PooledConnection<PgConnectionManager>;

pub fn establish_database_pool(database_url: &str) -> PgPool {
    let database_manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(15)
        .build(database_manager)
        .expect("Failed to create database pool")
}

pub mod schema;
pub mod models;
pub mod queries;
