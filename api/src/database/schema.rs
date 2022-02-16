table! {
    servers (id) {
        id -> Varchar,
        server_owner_id -> Varchar,
        is_verified -> Bool,
        is_promoted -> Bool,
        address -> Varchar,
        description -> Varchar,
        youtube_video -> Varchar,
        banner_gif_url -> Varchar,
        country -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    servers_links (uuid) {
        uuid -> Uuid,
        server_id -> Varchar,
        key -> Varchar,
        value -> Varchar,
    }
}

table! {
    servers_ping_record (uuid) {
        uuid -> Uuid,
        server_id -> Varchar,
        online -> Bool,
        motd -> Varchar,
        player_count -> Int4,
        ping -> Int4,
        version -> Varchar,
        recorded_at -> Timestamp,
    }
}

table! {
    servers_ping_record_player (player_uuid) {
        player_uuid -> Uuid,
        ping_record_uuid -> Uuid,
    }
}

table! {
    servers_tags (uuid) {
        uuid -> Uuid,
        server_id -> Varchar,
        tag -> Varchar,
    }
}

table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        email -> Varchar,
        profile_picture_url -> Varchar,
        is_admin -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

joinable!(servers -> users (server_owner_id));
joinable!(servers_links -> servers (server_id));
joinable!(servers_ping_record -> servers (server_id));
joinable!(servers_ping_record_player -> servers_ping_record (ping_record_uuid));
joinable!(servers_tags -> servers (server_id));

allow_tables_to_appear_in_same_query!(
    servers,
    servers_links,
    servers_ping_record,
    servers_ping_record_player,
    servers_tags,
    users,
);
