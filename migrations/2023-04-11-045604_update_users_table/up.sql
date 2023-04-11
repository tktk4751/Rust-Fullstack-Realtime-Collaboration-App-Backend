-- users table
ALTER TABLE users ALTER COLUMN id TYPE UUID USING id::UUID;
ALTER TABLE users RENAME COLUMN password_hash TO password;
ALTER TABLE users ADD COLUMN username VARCHAR;

-- tasks table
ALTER TABLE tasks ALTER COLUMN assignee_id TYPE UUID USING assignee_id::UUID;

-- projects table
ALTER TABLE projects ALTER COLUMN owner_id TYPE UUID USING owner_id::UUID;
