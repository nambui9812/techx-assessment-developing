## Getting Started
We are creating a backend that serve users to create and provision their workspaces.
Workspace should have a name, a description in free text, create and update time and owner
user id. Workspaces have 4 statuses:
- Inactive
- Active
- Expired
- Provisioning
We need API endpoint to:
- Get workspace by ID.
- List all workspaces.
- Create workspace.
- Update workspace status.
The user id for each of these operations are to be extracted from HTTP Header.

Please follow below instruction to run the project.

### Requirements
* With Docker
```
Docker
Docker-Compose
```
* Without Docker
```
You have to install Rust and PostgreSQL locally in your machine
```

### Installation
```
git clone https://github.com/nambui9812/techx-assessment-developing.git
cd techx-assessment-developing
```
* With Docker
```
docker compose up -d
```
* Without Docker
1. Start PostgreSQL server (If needed)
```
sudo systemctl start postgresql
```
2. Login
```
sudo -u postgres psql postgres
```
3. Setup user and database
```
CREATE ROLE user LOGIN PASSWORD 'password';
CREATE DATABASE db WITH OWNER = user;
\q
```
4. Login with new user and type in the password when prompted
```
psql -h localhost -d db -U user
```
5. Setup tables
```
Run all commands inside techx-assessment-developing/db/init.sql
Update the DATABASE_URL inside .env file to "postgres://user:password@localhost:5432/db"
```
6. Start the Axum server
```
cargo run
```
### Usage
* APIs
1. Get `/` - Return `Hello world`
2. Get `/users` - Return initialized users data
```
{
	"success": true,
	"data": [
		{ "id": 1, "username": "user1" },
		{ "id": 2, "username": "user2" },
		{ "id": 3, "username": "user3" }
	]
}
```
3. Post `/` - Create new user
```
{
	"username": "username"
}
```
> /workspaces domain require a custom header `x-ower`:`id_of_user`
4. Get `/workspaces`  - Return all workspace of a user
5. Get `/workspaces/:workspace_id` - Return specific workspace if owning
6. Post `/workspaces` - Create new workspace
```
{
	"name": "name",
	"description": "description"
}
```
7. Put `/workspaces/:workspace_id` - Update status of workspace if owning
```
{
	"status": "One of ["Inactive", "Active", "Expired", "Provisioning"]"
}
```