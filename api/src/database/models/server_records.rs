use crate::database::schema::{servers_ping_record, servers_ping_record_player};

use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Clone)]
pub struct ServersPingRecordModel {
    pub uuid: Uuid,
    pub server_id: String,
    
    pub online: bool,
    pub motd: String,
    pub player_count: i64,
    pub ping: i64,
    pub version: String,
    
    pub recorded_at: NaiveDateTime,
}

#[derive(Queryable, Debug, Clone)]
pub struct ServersPingRecordPlayerModel {
    pub player_uuid: Uuid,
    pub ping_record_uuid: Uuid,
}