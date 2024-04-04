-- Add down migration script here
DELETE FROM workspaces
WHERE name = 'name1';

DELETE FROM users
WHERE username = 'user1';

DELETE FROM users
WHERE username = 'user2';

DELETE FROM users
WHERE username = 'user3';
