use crate::database::queries::server::ServerQuery;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Server {
    pub id: String,

    pub owner: String, // TODO: I don't know what to do here...

    pub is_verified: bool,
    pub is_promoted: bool,

    pub address: String,
    pub description: String,
    pub youtube_video: String,
    pub banner_gif_url: String,
    pub country: String,

    pub links: Vec<ServerLink>,
    pub tags: Vec<String>,

    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerLink {
    key: String,
    value: String,
}

impl From<ServerQuery> for Server {
    fn from(q: ServerQuery) -> Self {
        let server = q.server;
        let links = q.links;
        let tags = q.tags;

        Self {
            id: server.id,
            owner: server.server_owner_id,
            is_verified: server.is_verified,
            is_promoted: server.is_promoted,
            address: server.address,
            description: server.description,
            youtube_video: server.youtube_video,
            banner_gif_url: server.banner_gif_url,
            country: server.country,

            links: vec![],
            tags: vec![],

            created_at: String::from(""),
        }
    }
}
