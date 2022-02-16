use crate::database::schema::{users};

use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Clone)]
pub struct User {
    pub id: String,
    
    pub username: String,
    pub email: String,
    pub profile_picture_url: String,
    
    pub is_admin: bool,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
