use crate::database::PgPooledConnection;
use crate::database::models::server::{ServerModel, ServerLinkModel, ServerTagModel};

use diesel::prelude::*;

#[derive(Debug)]
pub struct ServerQuery {
    pub server: ServerModel,
    pub links: Vec<ServerLinkModel>,
    pub tags: Vec<ServerTagModel>,
}

impl ServerQuery {
    pub fn get_by_id(connection: PgPooledConnection, requested_server_id: &str) -> Result<Option<ServerQuery>, diesel::result::Error> {
        use crate::database::schema::servers::dsl::*;
        use crate::database::schema::servers_links::{ dsl as servers_links_dsl, dsl::* };
        use crate::database::schema::servers_tags::{ dsl as servers_tags_dsl, dsl::* };

        let servers_query_results = servers
            .filter(id.eq(requested_server_id))
            .limit(1)
            .load::<ServerModel>(&connection)?;

        let server = match servers_query_results.first() {
            None => { return Ok(None) },
            Some(server) => server,
        };

        let server_links = servers_links
            .filter(servers_links_dsl::server_id.eq(requested_server_id))
            .load::<ServerLinkModel>(&connection)?;

        let server_tags = servers_tags
            .filter(servers_tags_dsl::server_id.eq(requested_server_id))
            .load::<ServerTagModel>(&connection)?;

        Ok(Some(Self {
            server: server.clone(),
            links: server_links,
            tags: server_tags,
        }))
    }
}
