CREATE OR REPLACE FUNCTION update_updated_at_column() RETURNS TRIGGER AS $$
    BEGIN
        IF (
            NEW IS DISTINCT FROM OLD AND
            NEW.updated_at IS NOT DISTINCT FROM old.updated_at
        ) THEN
            NEW.updated_at = current_timestamp;
        END IF;
        RETURN NEW;
    END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION generate_id(size INT) RETURNS TEXT AS $$
    DECLARE
        characters TEXT = 'abcdefghijklmnopqrstuvwxyz1234567890';
        length INT = length(characters);
        output TEXT = '';
        i INT = 0;
    BEGIN
        WHILE i < size LOOP
            output = output || SUBSTRING(characters FROM (random() * length)::integer FOR 1);
            i = i + 1;
        END LOOP;
        RETURN output;
    END
$$ LANGUAGE 'plpgsql';

CREATE TABLE IF NOT EXISTS users (
    id VARCHAR PRIMARY KEY DEFAULT generate_id(12),

    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    profile_picture_url VARCHAR NOT NULL,

    -- TODO: Store OAuth stuff for GitHub and Google

    is_admin BOOLEAN NOT NULL DEFAULT FALSE,

    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at TIMESTAMP DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS servers (
    id VARCHAR PRIMARY KEY DEFAULT generate_id(12),

    server_owner_id VARCHAR NOT NULL,

    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    is_promoted BOOLEAN NOT NULL DEFAULT FALSE,

    address VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    youtube_video VARCHAR NOT NULL,
    banner_gif_url VARCHAR NOT NULL,
    country VARCHAR NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at TIMESTAMP DEFAULT NULL,

    CONSTRAINT fk_servers_to_users_1 FOREIGN KEY(server_owner_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS servers_links(
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    server_id VARCHAR NOT NULL,

    key VARCHAR NOT NULL,
    value VARCHAR NOT NULL,

    CONSTRAINT fk_servers_links_to_servers_1 FOREIGN KEY(server_id) REFERENCES servers(id)
);

CREATE TABLE IF NOT EXISTS servers_tags(
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    server_id VARCHAR NOT NULL,

    tag VARCHAR NOT NULL,

    CONSTRAINT fk_servers_tags_to_servers_1 FOREIGN KEY(server_id) REFERENCES servers(id)
);

CREATE TABLE IF NOT EXISTS servers_ping_record(
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    server_id VARCHAR NOT NULL,

    online BOOLEAN NOT NULL,
    motd VARCHAR NOT NULL,
    player_count INT NOT NULL,
    ping INT NOT NULL,
    version VARCHAR NOT NULL,

    recorded_at TIMESTAMP NOT NULL DEFAULT current_timestamp,

    CONSTRAINT fk_servers_ping_record_to_servers_1 FOREIGN KEY(server_id) REFERENCES servers(id)
);

CREATE TABLE IF NOT EXISTS servers_ping_record_player(
    player_uuid UUID PRIMARY KEY,
    ping_record_uuid UUID NOT NULL,

    CONSTRAINT fk_servers_ping_record_player_to_ping_record_1 FOREIGN KEY(ping_record_uuid) REFERENCES servers_ping_record(uuid)
);

CREATE TRIGGER update_users_updated_at_column BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

CREATE TRIGGER update_servers_updated_at_column BEFORE UPDATE ON servers
    FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();
