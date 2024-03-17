# foosball-elo-tracker

## Development

### Getting Started

#### Server

Create a `.env` file based on `.env.example` and fill in the correct values.

To run the server in development mode (i.e. to watch for file changes), run the following command in the `/server` directory:

```shell
cargo watch -x run
```

Alternatively, you can also specify a directory to watch changes in, i.e. `src`:

```shell
cargo watch -w src -x run
```

### Database Migrations

When updating the database structure, first create migrations by running the following command in the `/server` directory:

```shell
diesel migration generate create_events
```

Once you've updated the relevant `up.sql` and `down.sql` files, run the migration with the following command in the `/server` directory:

```shell
diesel migration run
```

## Running the Application

### Server

To start the server, run the following command in the `/server` directory:

```shell
cargo run
```
