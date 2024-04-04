-- Add down migration script here
DROP TRIGGER IF EXISTS update_workspace
ON workspaces;

DROP FUNCTION IF EXISTS update_on_update_time_workspace;

DROP TABLE IF EXISTS workspaces;

DROP TYPE IF EXISTS status;

DROP TABLE IF EXISTS users;
