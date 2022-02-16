DROP TRIGGER IF EXISTS update_users_updated_at_column ON users;
DROP TRIGGER IF EXISTS update_servers_updated_at_column ON servers;

DROP TABLE IF EXISTS
    servers_ping_record_player,
    servers_ping_record,
    servers_links,
    servers_tags,
    servers,
    users;

DROP FUNCTION IF EXISTS generate_id;
DROP FUNCTION IF EXISTS update_updated_at_column;
