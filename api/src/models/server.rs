use crate::database::models::server::ServerModel;
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

    pub created_at: String,
}

impl From<ServerModel> for Server {
    fn from(m: ServerModel) -> Self {
        Self {
            id: m.id,
            owner: m.server_owner_id,
            is_verified: m.is_verified,
            is_promoted: m.is_promoted,
            address: m.address,
            description: m.description,
            youtube_video: m.youtube_video,
            banner_gif_url: m.banner_gif_url,
            country: m.country,
            created_at: m.created_at,
        }
    }
}
