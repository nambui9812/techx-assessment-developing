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
1. Start 2 Docker services
```
docker compose up -d
```
* Without Docker
1. Start PostgreSQL server (If needed)
```
sudo systemctl start postgresql
```
2. Run migration
```
Follow the Migration instruction below
```
3. Start the Axum server
```
cargo build
cargo run
```
### Migration
* With Docker
```
Migrations will run automatically
```
* Without Docker (Or for tests)
* Requirements
1. PostgreSQL server is running
2. .env file has valid `DATABASE_URL` field
3. `sqlx-cli` installed
```
sudo apt install pkg-config
cargo install sqlx-cli
Or cargo install sqlx-cli --no-default-features --features native-tls,postgres	(Only for postgres)
```
* Steps
1. Create database and run migrations
```
sqlx database create	(If database not exists yet when starting without Docker)
sqlx migration run --source ./db/migrations-for-tests (for test)
```
2. (Optional) Revert database if needed
```
sqlx migration revert --source ./db/migrations-for-tests (for test) (Revert step by step)
sqlx database drop		(Drop the database)
```
### Usage
* `/users` APIs
1. GET `/` - Return `Hello world`
2. GET `/users` - Return initialized users data
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
3. POST `/users` - Create new user
```
{
	"username": "username"
}
```
* `/workspaces` APIs
> /workspaces domain require a custom header `x-ower`:`id_of_user`
1. GET `/workspaces`  - Return all workspace of a user
2. GET `/workspaces/:workspace_id` - Return specific workspace if owning
3. POST `/workspaces` - Create new workspace
```
{
	"name": "name",
	"description": "description"
}
```
4. PUT `/workspaces/:workspace_id` - Update status of workspace if owning
```
{
	"status": "One of ["Inactive", "Active", "Expired", "Provisioning"]"
}
```
### Test
* Requirements
1. PostgreSQL server is running (Locally with valid config based on DATABASE_URL in .env file Or from Docker-Compose `docker compose up -d pg`)
2. Migrations were run, fake data was created
```
sqlx migrate run --source ./db/migrations-for-tests (for test)
```
* Run
```
cargo test
```
* Coverages:
1. GET `/workspaces`
	- test_database_connectivity
	- test_get_workspaces_successfully
	- test_get_workspaces_failed_due_to_lack_of_ower_id_header
2. GET `/workspaces/:workspace_id`
	- test_get_workspace_by_id_successfully
	- test_get_workspace_by_id_failed_due_to_lack_of_ower_id_header
	- test_get_workspace_by_id_failed_due_to_invalid_id_param
3. POST `/workspaces`
	- test_create_workspace_successfully
	- test_create_workspace_failed_due_to_lack_of_ower_id_header
	- test_create_workspace_failed_due_to_lack_of_name_field
4. PUT `/workspaces/:workspace_id`
	- test_update_workspace_status_successfully
	- test_update_workspace_status_failed_due_to_lack_of_ower_id_header
	- test_update_workspace_status_failed_due_to_invalid_id_param
	- test_update_workspace_status_failed_due_to_invalid_status_field