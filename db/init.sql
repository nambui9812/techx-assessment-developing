DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id INT GENERATED ALWAYS AS IDENTITY,
    username VARCHAR(255) NOT NULL,
    PRIMARY KEY(id)
);

DROP TYPE IF EXISTS status;
CREATE TYPE status AS ENUM (
    'Inactive',
    'Active',
    'Expired',
    'Provisioning'
);

DROP TABLE IF EXISTS workspaces;
CREATE TABLE workspaces (
    id INT GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    create_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    status status,
    owner_id INT,
    PRIMARY KEY(id),
    CONSTRAINT fk_user
        FOREIGN KEY(owner_id)
            REFERENCES users(id)
            ON DELETE CASCADE
);

CREATE OR REPLACE FUNCTION update_on_update_time_workspace()
RETURNS TRIGGER AS $$
BEGIN
    NEW.update_time = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_workspace
    BEFORE UPDATE
    ON
        workspaces
    FOR EACH ROW
EXECUTE PROCEDURE update_on_update_time_workspace();

INSERT INTO users(username)
VALUES('user1'),
      ('user2'),
      ('user3');
