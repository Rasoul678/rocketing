# Rocketing

This repo is an experimental try to explore [Rocket](https://rocket.rs/) and [Diesel](https://diesel.rs/) crates. I am using postgresql as database with docker.

### Steps:

- 1

```powershell Go inside project
cd ./directory
```

- 2

```powershell spin up docker
docker-compose up
```

#### Add diesel-cli by either:

- 3

```powershell Install the CLI tool with curl
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh
```

#### Or cargo:

```powershell Install the tool with cargo
cargo install diesel_cli --no-default-features --features postgres
```

- 4 (choose whatever db you prefer. see other options!)

#### - By default diesel CLI depends on [libpq](https://www.postgresql.org/docs/current/libpq.html) for the PostgreSQL backend

- 5

```powershell And run:
diesel setup
```

- 6

```powershell Then run:
diesel migration generate "table name"
```

- 7
- Next, write the SQL for migrations

- 8

```powershell Finally run:
diesel migration run
```

### Continue reading...

For more instructions read [this](https://diesel.rs/guides/getting-started) article.
