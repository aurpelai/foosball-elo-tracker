# Backend

## Getting Started

Create a `.env` file based on `.env.example` and fill in the correct values.

### Dependencies

On macOS you can install Rust, PostgreSQL and Diesel CLI by running the following commands:

```shell
brew install postgresql@16 rust
cargo install diesel_cli --no-default-features --features postgres
```

It might also be beneficial to install the `cargo-watch` library so you can have hot reloading of changed files.

```shell
cargo install cargo-watch
```

Finally, install dependencies by running:

```shell
cargo build
```

If the above command fails with message `ld: library 'pq' not found`, run

```shell
cargo clean
brew link --force libpq
cargo build
```

### Database

Create a database using the below commends and add the connection values to your `.env` file.

```shell
createdb database-name
psql -c "CREATE ROLE myuser WITH LOGIN PASSWORD 'mypassword';" database-name
psql -c "GRANT ALL ON SCHEMA public TO myuser;" database-name
```

After you've successfully created the database, run migrations with

```shell
diesel migration run
```

#### Database Migrations

When updating the database structure, first create migrations by running the following command:

```shell
diesel migration generate create_events
```

Once you've updated the relevant `up.sql` and `down.sql` files, run the migration with the following command:

```shell
diesel migration run
```

## Running the Application

### Development

To run the server in development mode (i.e. to watch for file changes), run the following command:

```shell
cargo watch -x run
```

Alternatively, you can also specify a directory to watch changes in, i.e. `src`:

```shell
cargo watch -w src -x run
```

### Production

To start the server, run the following command:

```shell
cargo run
```
