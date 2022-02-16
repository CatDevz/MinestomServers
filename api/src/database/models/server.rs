use crate::database::schema::{servers, servers_links, servers_tags};

use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Clone)]
pub struct ServerModel {
    pub id: String,

    pub server_owner_id: String,

    pub is_verified: bool,
    pub is_promoted: bool,
    
    pub address: String,
    pub description: String,
    pub youtube_video: String,
    pub banner_gif_url: String,
    pub country: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Clone)]
pub struct ServerLinkModel {
    pub uuid: Uuid,
    pub server_id: String,
    pub key: String,
    pub value: String,
}

#[derive(Queryable, Debug, Clone)]
pub struct ServerTagModel {
    pub uuid: Uuid,
    pub server_id: String,
    pub tag: String,
}
