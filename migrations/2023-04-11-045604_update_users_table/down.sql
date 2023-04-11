-- users table
ALTER TABLE users ALTER COLUMN id TYPE INTEGER USING id::INTEGER;
ALTER TABLE users RENAME COLUMN password TO password_hash;
ALTER TABLE users DROP COLUMN username;

-- tasks table
ALTER TABLE tasks ALTER COLUMN assignee_id TYPE INTEGER USING assignee_id::INTEGER;

-- projects table
ALTER TABLE projects ALTER COLUMN owner_id TYPE INTEGER USING owner_id::INTEGER;
