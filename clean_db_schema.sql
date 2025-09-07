-- Add migration script here
-- This migration will be updated a hundred times,
-- whenever a new migration is created to fix something
-- so when creating a new database, the only migration you need to run
-- is this one

--- USERS ---

CREATE TABLE users (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	username VARCHAR(128) UNIQUE NOT NULL,
	password VARCHAR(128) NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TYPE user_availability AS ENUM ('Online', 'Idle', 'DND', 'Invisible');

CREATE TABLE public_images (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	url VARCHAR(1024) NOT NULL UNIQUE,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE user_profile (
	id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
	display_name VARCHAR(255) NOT NULL,
	avatar_url UUID REFERENCES public_images(id) ON DELETE SET NULL,
	banner_url UUID REFERENCES public_images(id) ON DELETE SET NULL,
	availablility user_availability DEFAULT 'Online',
	status VARCHAR(128),
	status_until TIMESTAMP,
	bio TEXT,
	-- TODO: use redis for last seen
	last_seen TIMESTAMP NOT NULL DEFAULT NOW(),
	online BOOLEAN NOT NULL DEFAULT FALSE,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TYPE connection_type AS ENUM ('IncomingRequest', 'OutgoingRequest', 'IncomingBlock', 'OutgoingBlock', 'Friend');

CREATE TABLE user_connections (
	-- source user
	user1_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	-- target user
	user2_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	type CONNECTION_TYPE NOT NULL,
	nickname VARCHAR(32),
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

	CONSTRAINT uc_order_user_pair CHECK (user1_id < user2_id),
	PRIMARY KEY (user1_id, user2_id)
);

--- USERS:END ---

--- REACTIONS ---

-- TODO: reaction packs?

CREATE TABLE reaction_packs (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	name VARCHAR(32) NOT NULL,
	search_keywords VARCHAR(32)[] CHECK (array_length(search_keywords, 1) <= 10), -- for searching entire packs
	created_by UUID REFERENCES users(id) ON DELETE SET NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE reactions (
	id UUID GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	created_by UUID NOT NULL REFERENCES users(id) ON DELETE SET NULL,
	name VARCHAR(255) NOT NULL,
	reaction_url VARCHAR(1024) NOT NULL,
	is_discoverable BOOLEAN DEFAULT FALSE,
	search_keywords VARCHAR(32)[] CHECK (array_length(search_keywords, 1) <= 10), -- for searching individual reaction in pack
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE pack_reactions (
	pack_id UUID PRIMARY KEY REFERENCES reaction_packs(id) ON DELETE CASCADE,
	reaction_id UUID PRIMARY KEY REFERENCES reactions(id) ON DELETE CASCADE,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW()

	PRIMARY KEY (pack_id, reaction_id)
);

CREATE TABLE user_reaction_packs (
	user_id UUID REFERENCES users(id) ON DELETE CASCADE,
	reaction_pack_id UUID REFERENCES reaction_packs(id) ON DELETE CASCADE,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

	PRIMARY KEY (user_id, reaction_pack_id)
);

--- REACTIONS:END ---

--- CONVERSATIONS ---

CREATE TABLE conversations (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	title VARCHAR(255),
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE conversation_members (
	conversation_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
	user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE, -- set null instead?
	user_note TEXT,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	PRIMARY KEY (conversation_id, user_id)
);

--- CONVERSATIONS:END ---

CREATE TYPE message_status AS ENUM ('Sent', 'Delivered', 'Read', 'Failed');

-- TODO: starred messages
CREATE TABLE user_messages (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	reply_to_id UUID REFERENCES user_messages(id) ON DELETE SET NULL,
	has_reply_to_id BOOLEAN NOT NULL DEFAULT FALSE,
	conversation_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
	sender_id UUID REFERENCES users(id) ON DELETE SET NULL,
	content TEXT,
	is_markdown BOOLEAN NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	status MESSAGE_STATUS NOT NULL DEFAULT 'Sent',
	-- for fk in user reactions
	CONSTRAINT uc_id_sender UNIQUE (id, sender_id),
	CONSTRAINT msg_not_own_reply CHECK (id != reply_to_id),
	FOREIGN KEY (conversation_id, sender_id) REFERENCES conversation_members(conversation_id, user_id)
);

CREATE TABLE attachments (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	-- TODO: periodically check if null, then remove from s3
	message_id UUID REFERENCES user_messages(id) ON DELETE SET NULL,
	mime_type VARCHAR(256),
	size_bytes BIGINT NOT NULL,
	url VARCHAR(128) NOT NULL UNIQUE,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE user_message_attachments (
	message_id UUID REFERENCES user_messages(id) ON DELETE SET NULL,
	attachment_id UUID REFERENCES attachments(id) ON DELETE RESTRICT, -- if this happens, something is wrong
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	PRIMARY KEY (message_id, attachment_id)
);

CREATE TABLE pinned_messages (
	conversation_id UUID NOT NULL,
	message_id UUID NOT NULL,
	pinned_by_id UUID REFERENCES users(id) ON DELETE SET NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

	FOREIGN KEY (conversation_id, message_id) REFERENCES user_messages(conversation_id, id) ON DELETE CASCADE,
	FOREIGN KEY (conversation_id, pinned_by_id) REFERENCES conversation_members(conversation_id, user_id),
	PRIMARY KEY (conversation_id, message_id)
);

-- TODO: implement reaction pack for user conversations
CREATE TABLE user_message_reactions (
	message_id UUID NOT NULL REFERENCES user_messages(id) ON DELETE SET NULL,
	reactor_id UUID NOT NULL REFERENCES users(id) ON DELETE SET NULL,
	reaction_id UUID NOT NULL REFERENCES reactions(id) ON DELETE SET NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	FOREIGN KEY (message_id, reactor_id) REFERENCES user_messages(id, sender_id),
	-- CONSTRAINT uc_user_message_reaction UNIQUE (message_id, reaction_id)
	PRIMARY KEY (message_id, reaction_id)
);

-- @jimit --- reviewed till here - sept 3 2025

-- TODO: multi people conversation groups, and it has polls
-- TODO: pinned messages in channels/guilds

-- TODO: audit logs?
-- TODO: bans
-- TODO: guild events and channel events (joined message/event, left message/event)
CREATE TABLE guilds (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	name VARCHAR(128) NOT NULL,
	avatar_id UUID REFERENCES public_images(id) ON DELETE SET NULL,
	banner_id UUID REFERENCES public_images(id) ON DELETE SET NULL,
	traits VARCHAR(128)[] CHECK (array_length(traits, 1) < 5),
	description TEXT,
	-- TODO: separate table for guild events?
	join_message BOOLEAN NOT NULL DEFAULT FALSE,
	left_message BOOLEAN NOT NULL DEFAULT FALSE,
	kicked_message BOOLEAN NOT NULL DEFAULT FALSE,
	banned_message BOOLEAN NOT NULL DEFAULT FALSE,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE users_on_guilds (
	user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	guild_id UUID NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
	nickname VARCHAR(256), -- unique?
	-- TODO: role?
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	-- CONSTRAINT uc_users_on_guilds UNIQUE (user_id, guild_id),
	-- TODO: invert pk
	PRIMARY KEY (guild_id, user_id)
);

CREATE TABLE guild_bans (
	banned_by UUID NOT NULL REFERENCES users(id) ON DELETE SET NULL,
	-- TODO: role of banner when banned?
	banned_user UUID REFERENCES users(id) ON DELETE CASCADE,
	guild_id UUID NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
	reason TEXT,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	FOREIGN KEY (guild_id, banned_by) REFERENCES users_on_guilds(guild_id, user_id),
	PRIMARY KEY (guild_id, banned_user)
);

-- TODO: automod

CREATE TABLE guild_roles (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	guild_id UUID NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
	role_name VARCHAR(128) NOT NULL,
	color_rgb CHAR(6) NOT NULL CHECK color_rgb ~* '^[0-9A-F]{6}$',
	-- TODO: role icon?
	mentionable BOOLEAN NOT NULL DEFAULT FALSE,
	default BOOLEAN NOT NULL DEFAULT TRUE,
	priority SMALLINT NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	CONSTRAINT uc_guild__role_name_unique UNIQUE (guild_id, role_name),
	CONSTRAINT uc_id_guild_id_composite_unique_for_fk UNIQUE (guild_id, id)
);

CREATE TABLE user_roles_on_guild (
	user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	role_id UUID NOT NULL,
	-- guild id for ensuring user is present on the guild, and when he leaves, cascade delete role
	guild_id UUID NOT NULL,

	FOREIGN KEY (guild_id, user_id) REFERENCES users_on_guilds(guild_id, user_id) ON DELETE CASCADE,
	FOREIGN KEY (role_id, guild_id) REFERENCES guild_roles(guild_id, id) ON DELETE CASCADE,
	PRIMARY KEY (user_id, role_id)
);

CREATE TABLE guild_role_permissions (
	role_id UUID NOT NULL REFERENCES guild_roles(id),

	-- TODO: missing more granular permissions from revolt.

	-- General Server Permissions
	--	1. View Channels
	-- 	2. Manage Channels
	-- 	3. Manage Roles
	-- 	4. Create Expressions
	-- 	5. Manage Expressions
	-- 	6. View Audit Log
	-- 	7. Manage Webhooks
	-- 	8. Manage Server

	-- Membership Permissions
	-- 	9. Create Invite
	-- 10. Change Nickname
	-- 11. Manage Nicknames
	-- 12. Kick, Approve and Reject Members
	-- 13. Ban Members
	-- 14. Timeout Members
	-- Text Channel Permissions
	-- 15. Send Messages and Create Posts
	-- 16. Send Messages in Threads and Posts
	-- 17. Create Public Threads
	-- 18. Create Private Threads
	-- 19. Embed Links
	-- 20. Attach Files
	-- 21. Add Reactions
	-- 22. Use External Emoji
	-- 23. Use External Stickers
	-- 24. Mention @everyone, @here, and All Roles
	-- 25. Manage Messages
	-- 26. Manage Threads and Posts
	-- 27. Read Message History
	-- 28. Send Text-to-Speech Messages
	-- 29. Send Voice Messages
	-- 30. Create Polls

	-- Voice Channel Permissions
	-- 31. Connect
	-- 32. Speak
	-- 33. Video
	-- 34. Use Soundboard
	-- 35. Use External Sounds
	-- 36. Use Voice Activity
	-- 37. Priority Speak
	-- 38. Mute Members
	-- 39. Deafen Members
	-- 40. Move Members
	-- 41. Set Voice Channel Status

	-- App Permissions
	-- 42. Use Application Commands
	-- 43. Use Activities
	-- 44. Use External Apps

	-- Event Permissions
	-- 45. Create Events
	-- 46. Manage Events

	-- Advanced Permissions
	-- 47. Administrator

	-- TODO: virtual generated fields for each permission

	permissions BIGINT NOT NULL DEFAULT 0
);

CREATE TABLE channel_categories (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	name VARCHAR(256) NOT NULL,
	guild_id UUID NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

	CONSTRAINT uc_channel_category_name_for_guild UNIQUE (guild_id, name)
);

CREATE TYPE channel_type as ENUM ('Text', 'Audio', 'Video', 'Forum');

-- TODO: private flag in channels for some roles
CREATE TABLE channels (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	name VARCHAR(128) NOT NULL,
	guild_id UUID NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
	type CHANNEL_TYPE NOT NULL DEFAULT 'Text',
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	CONSTRAINT uc_channel_name_per_guild UNIQUE (
		guild_id,
		name
	)
);

CREATE TABLE post_channel_settings (
	channel_id UUID PRIMARY KEY REFERENCES channels(id) ON DELETE CASCADE,
	guidelines TEXT,
	-- TODO: convert these in bitflag
	-- TODO: default reaction?
	require_tags BOOLEAN DEFAULT FALSE,
	default_list_view BOOLEAN DEFAULT TRUE,
	default_sort_by_activity BOOLEAN DEFAULT TRUE,
	default_tag_match_some BOOLEAN DEFAULT TRUE,

	auto_delete_at TIMESTAMP DEFAULT NOW() + '3 days'::INTERVAL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE post_channel_tags (
	channel_id UUID PRIMARY KEY REFERENCES post_channel_settings(channel_id) ON DELETE CASCADE,
	name VARCHAR(32) NOT NULL,
	mods_only_apply BOOLEAN DEFAULT FALSE,

	CONSTRAINT uc_channel_tags UNIQUE (channel_id, name)
);

CREATE TABLE posts (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	guild_id UUID NOT NULL,
	channel_id UUID NOT NULL,
	title VARCHAR(128) NOT NULL,
	content TEXT NOT NULL,
	has_tags BOOLEAN NOT NULL DEFAULT FALSE,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

	FOREIGN KEY (guild_id, channel_id) REFERENCES channels(guild_id, id) ON DELETE CASCADE
);

-- TODO: post notifications, etc?
-- TODO: post guildelines
-- TODO: post reactions

CREATE TABLE guild_post_tags ();

CREATE TABLE post_followers ();

CREATE TYPE message_type AS ENUM ('TextOrMedia', 'Poll');

CREATE TABLE channel_messages (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	channel_id UUID REFERENCES channels(id) ON DELETE CASCADE,
	parent_thread_id UUID,
	post_id UUID REFERENCES posts(id) ON DELETE CASCADE,
	sent_by UUID REFERENCES users(id) ON DELETE SET NULL,
	is_markdown BOOLEAN NOT NULL,
	content TEXT,
	type MESSAGE_TYPE NOT NULL DEFAULT 'TextOrMedia',
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

	CONSTRAINT either_parent_channel_or_post_or_thread CHECK (
		(channel_id IS NOT NULL AND thread_id IS NULL AND post_id IS NULL)
		OR
		(thread_id IS NOT NULL AND channel_id IS NULL AND post_id IS NULL)
		OR
		(post_id IS NOT NULL AND channel_id IS NULL AND thread_id IS NULL)
	),
	CONSTRAINT message_thread_circular_ref CHECK (id !=  parent_thread_id),
	-- for fk in polls
	-- CONSTRAINT uc_channel_id_id UNIQUE (channel_id, id),
	CONSTRAINT message_null_for_poll CHECK ((type = 'Poll' AND content IS NULL) OR type != 'Poll'),
	-- unique constraint for threads, to constraint that there is no thread emerging from a post message
	CONSTRAINT uc_id_post_id UNIQUE (id, post_id),
	-- TODO index on (channel_id, id) for FK on polls and other tables
	INDEX idx_channel_messages (channel_id, post_id, thread_id, created_at)
);

CREATE TYPE thread_notification_settings AS ENUM ('All', 'Mentions', 'None');

-- should threads be implicit?
-- FIXME: posts should not have thread messages
CREATE TABLE threads (
	id UUID PRIMARY KEY REFERENCES channel_messages(id) ON DELETE CASCADE, -- also parent_message_id
	created_by UUID REFERENCES users(id) ON DELETE SET NULL,
	auto_delete_at TIMESTAMP NOT NULL DEFAULT NOW() + '3 days'::INTERVAL,
	-- TODO: thread closed/archived, etc.
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

	FOREIGN KEY (id, NULL) REFERENCES channel_messages(id, post_id)
);

CREATE TABLE thread_members (
	thread_id UUID NOT NULL REFERENCES threads(id) ON DELETE CASCADE,
	indefinitely_muted BOOLEAN NOT NULL DEFAULT FALSE,
	muted_until TIMESTAMP,
	notification_enabled_for thread_notification_settings NOT NULL DEFAULT 'All',
	-- having guild_id as well, so when user leaves guild, automatically cascades
	guild_id UUID NOT NULL,
	user_id UUID NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

	FOREIGN KEY (guild_id, user_id) REFERENCES users_on_guilds(guild_id, user_id) ON DELETE CASCADE,
	PRIMARY KEY (thread_id, user_id),

	INDEX idx_thread (thread_id)
);

ALTER TABLE channel_messages ADD CONSTRAINT fk_parent_thread FOREIGN KEY (parent_thread_id) REFERENCES threads(id) ON DELETE CASCADE;

-- CREATE TABLE thread_members 

--- POLLS ---

CREATE TABLE polls (
	id UUID PRIMARY KEY REFERENCES channel_messages(id) ON DELETE CASCADE,
	-- channel_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
	start_time TIMESTAMP NOT NULL,
	end_time TIMESTAMP NOT NULL,
	is_anonymous BOOLEAN NOT NULL DEFAULT FALSE,
	is_multiple_choice BOOLEAN NOT NULL DEFAULT FALSE,
	-- is_quiz BOOLEAN NOT NULL DEFAULT FALSE,
	-- answer_id UUID,
	question TEXT NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

	CONSTRAINT start_time_gt_end_time CHECK (start_time < end_time)
	-- TODO: need to split fk def in alter table
	-- FOREIGN KEY (id, answer_id) REFERENCES poll_options(poll_id, id) ON DELETE CASCADE,
	-- will not work
	-- CONSTRAINT quiz_has_answer CHECK ((is_quiz AND answer_id IS NULL) OR (not is_quiz AND answer_id IS NOT NULL))
);

CREATE TABLE poll_options (
	id UUID DEFAULT gen_random_uuid() UNIQUE,
	poll_id UUID REFERENCES polls (id) ON DELETE CASCADE,
	content VARCHAR(255) NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

	CONSTRAINT uc_option_content_on_poll UNIQUE (poll_id, content),
	PRIMARY KEY (poll_id, id) -- for fk constraint on poll_votes
);

CREATE TABLE poll_votes (
	poll_message_id UUID NOT NULL,
	option_id UUID NOT NULL,
	voter_id UUID REFERENCES users(id) ON DELETE CASCADE,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

	FOREIGN KEY (poll_message_id, option_id) REFERENCES poll_options(poll_id, id) ON DELETE CASCADE,
	CONSTRAINT uc_poll_vote_by_user UNIQUE (poll_message_id, voter_id, option_id)
);

CREATE UNIQUE INDEX ux_single_vote
ON poll_votes(poll_message_id, voter_id)
WHERE NOT EXISTS (
	SELECT 1 FROM polls WHERE polls.id = poll_votes.poll_message_id AND polls.is_multiple_choice
);


--- POLLS:END ---

-- TODO: threads on channels
-- TODO: forums and posts, tags on posts

-- TODO: stickers

-- TODO: questions, single choice, multiple choice, dynamic answer matching (all valid selected, any valid selected, etc.)

-- FIX: better to have reply parent id in the same messages table
-- to avoid an extra join at the cost of extra storage
CREATE TABLE channel_replies (
	channel_id UUID NOT NULL REFERENCES channels(id) ON DELETE CASCADE,
	reply_to_id UUID NOT NULL REFERENCES channel_messages(id) ON DELETE SET NULL,
	message_id UUID NOT NULL UNIQUE REFERENCES channel_messages(id) ON DELETE CASCADE,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE guild_reactions (
	guild_id UUID NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
	reaction_id UUID NOT NULL REFERENCES reactions(id) ON DELETE CASCADE,
	adder_id UUID NOT NULL REFERENCES users(id) ON DELETE SET NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	-- CONSTRAINT uc_guild_reaction UNIQUE (guild_id, reaction_id)
	PRIMARY KEY (guild_id, reaction_id)
);

CREATE TABLE channel_message_reactions (
	channel_message_id UUID NOT NULL,
	guild_id UUID NOT NULL,
	reaction_id UUID NOT NULL,
	reactor_id UUID NOT NULL REFERENCES users(id) ON DELETE SET NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	FOREIGN KEY (guild_id, reaction_id) REFERENCES guild_reactions(guild_id, reaction_id) ON DELETE CASCADE
	-- PRIMARY KEY
);

CREATE TABLE sticker_set ();

-- TODO: hide profile photo, hide last online
-- TODO: devices, external apps
-- TODO: invites
-- TODO: lobbies, what are they?
-- TODO: custom themes
-- TODO: all of user settings, notification settings, permissions
-- TODO: server tags
-- TODO: soundboard? what is that?
-- TODO: guidelines
-- TODO: guild and channel invites, webhooks, permissions
-- TODO: channel settings
