-- Add migration script here
-- This migration will be updated a hundred times,
-- whenever a new migration is created to fix something
-- so when creating a new database, the only migration you need to run
-- is this one
CREATE TABLE users (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	username VARCHAR(128) UNIQUE NOT NULL,
	password VARCHAR(128) NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	online BOOLEAN NOT NULL DEFAULT FALSE,
	last_seen TIMESTAMP NOT NULL DEFAULT NOW(),
	is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
	profile_picture_url VARCHAR(512),
	status VARCHAR(128) DEFAULT 'Available'
);

CREATE TABLE user_conversations (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	user1_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
	user2_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
	deleted_by_user1 BOOLEAN NOT NULL DEFAULT FALSE,
	deleted_by_user2 BOOLEAN NOT NULL DEFAULT FALSE,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	last_message_at TIMESTAMP NOT NULL DEFAULT NOW(), -- for fetching list
	CONSTRAINT check_user_order CHECK (user1_id <= user2_id),
	CONSTRAINT uc_user_conversation_pair UNIQUE (user1_id, user2_id)
);

CREATE TABLE user_requests (
	sender_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	receiver_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	rejected BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE user_conversation_blocks (
	blocked_user BIGINT REFERENCES users(id) ON DELETE CASCADE,
	blocked_by_user BIGINT REFERENCES users(id) ON DELETE CASCADE
);

CREATE FUNCTION fn_delete_conversation_on_both_delete()
	RETURNS TRIGGER
	LANGUAGE plpgsql
AS
$$
BEGIN
	IF NEW.user1_id IS NULL AND NEW.user2_id IS NULL OR NEW.deleted_by_user1 AND NEW.deleted_by_user2 THEN
		DELETE FROM user_conversations WHERE id = NEW.id;
	END IF;
END
$$;

CREATE FUNCTION fn_user_conversations_prevent_null_insert()
	RETURNS TRIGGER
	LANGUAGE plpgsql
AS
$$
BEGIN
	IF NEW.user1_id IS NULL OR NEW.user2_id IS NULL THEN
		RAISE EXCEPTION 'user id cannot be NULL';
	END IF;
	RETURN NEW;
END;
$$;

CREATE TRIGGER user_conversations_delete_conversation_when_both_delete
AFTER UPDATE ON user_conversations
FOR EACH ROW
EXECUTE FUNCTION fn_delete_conversation_on_both_delete();

CREATE TRIGGER user_conversations_prevent_null_insert
BEFORE INSERT ON user_conversations
FOR EACH ROW
EXECUTE FUNCTION fn_user_conversations_prevent_null_insert();

-- TODO: poll in messages
-- TODO: starred messages
CREATE TABLE user_messages (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	-- TODO: check - is there a way to constraint sender id,
	-- so that it is always present in conversations `conversation_id`
	conversation_id BIGINT NOT NULL REFERENCES user_conversations(id) ON DELETE CASCADE,
	sender_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
	content TEXT,
	is_markdown BOOLEAN NOT NULL,
	sent_at TIMESTAMP NOT NULL DEFAULT NOW(),
	delivered BOOLEAN NOT NULL DEFAULT FALSE,
	read BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE FUNCTION fn_user_messages_prevent_null_insert()
	RETURNS TRIGGER
	LANGUAGE plpgsql
AS
$$
BEGIN
	IF NEW.sender_id IS NULL THEN
		RAISE EXCEPTION 'sender id cannot be NULL';
	END IF;
	RETURN NEW;
END;
$$;

CREATE FUNCTION fn_update_last_message_time_on_insert()
	RETURNS TRIGGER
	LANGUAGE plpgsql
AS
$$
BEGIN
	UPDATE user_conversations SET last_message_at = NOW() WHERE ID = NEW.conversation_id;
END
$$;
-- TODO: update last_message_at on delete

CREATE TRIGGER user_messages_prevent_null_insert
BEFORE INSERT ON user_messages
FOR EACH ROW
EXECUTE FUNCTION fn_user_messages_prevent_null_insert();

CREATE TRIGGER user_conversations_update_last_message_time
AFTER INSERT ON user_messages
EXECUTE FUNCTION fn_update_last_message_time_on_insert();

CREATE FUNCTION fn_attachments_prevent_null_insert()
	RETURNS TRIGGER
	LANGUAGE plpgsql
AS
$$
BEGIN
	IF NEW.message_id IS NULL THEN
		RAISE EXCEPTION 'message id should not be null';
	END IF;
	RETURN NEW;
END
$$;

CREATE TABLE attachments (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	-- FIXME: periodically check if null, then remove from s3
	message_id BIGINT REFERENCES user_messages(id) ON DELETE SET NULL,
	url VARCHAR(128) NOT NULL
);

CREATE TRIGGER attachments_prevent_null_insert
BEFORE INSERT ON attachments
FOR EACH ROW
EXECUTE FUNCTION fn_attachments_prevent_null_insert();

-- FIX: better to have reply parent id in the same messages table
-- to avoid an extra join at the cost of extra storage
CREATE TABLE user_replies (
	replied_to_message BIGINT REFERENCES user_messages(id) ON DELETE SET NULL,
	reply_message_id BIGINT NOT NULL REFERENCES user_messages(id) ON DELETE CASCADE
);

CREATE TABLE blocked_users (
	blocked_user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	blocked_by_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	CONSTRAINT uc_block_once UNIQUE (blocked_user_id, blocked_by_id)
);

CREATE TABLE guilds (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	name VARCHAR(128) NOT NULL,
	profile_picture_url VARCHAR(128),
	created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE users_on_guilds (
	user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	guild_id BIGINT NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
	joined_at TIMESTAMP NOT NULL DEFAULT NOW(),
	CONSTRAINT uc_users_on_guilds UNIQUE (user_id, guild_id)
);

CREATE TABLE channels (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	name VARCHAR(128) NOT NULL,
	guild_id BIGINT NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
	CONSTRAINT uc_channel_name_per_guild UNIQUE (
		guild_id,
		name
	)
);

CREATE TABLE channel_messages (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	channel_id BIGINT NOT NULL REFERENCES channels(id) ON DELETE CASCADE,
	is_markdown BOOLEAN NOT NULL,
	content TEXT NOT NULL,
	sent_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
	sent_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- ISSUE: these types of replies are complete chaos in a chatroom (good for
-- friends hangout like discord). reply threads and mentions like mattermost
-- are more ideal for having a meaningful conversations with only concerned
-- members joining the thread on their will, while still being publicly
-- available.
-- FIX: better to have reply parent id in the same messages table
-- to avoid an extra join at the cost of extra storage
CREATE TABLE channel_replies (
	channel_id BIGINT NOT NULL REFERENCES channels(id) ON DELETE CASCADE,
	reply_to_id BIGINT NOT NULL REFERENCES channel_messages(id) ON DELETE SET NULL,
	message_id BIGINT NOT NULL REFERENCES channel_messages(id) ON DELETE CASCADE
);

-- TODO: permissions, assigner
CREATE TABLE moderators (
	user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	guild_id BIGINT NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
	CONSTRAINT uc_moderator_on_guild UNIQUE (
		user_id,
		guild_id
	)
);

-- TODO: reactions
-- TODO: hide profile photo, hide last online
