-- Add up migration script here
INSERT INTO users(username)
VALUES('user1'),
      ('user2'),
      ('user3');

INSERT INTO workspaces(name, description, status, owner_id)
VALUES('name1', 'desciption1', 'Active', 1);
