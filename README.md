# foosball-elo-tracker

## Development

### Getting Started

#### Server

Create a `.env` file based on `.env.example` and fill in the correct values.

### Database Migrations

When updating the database structure, first create migrations by running.

```shell
diesel migration generate create_events
```

Once you've updated the relevant `up.sql` and `down.sql` files, run the migration with

```shell
diesel migration run
```

### Running the Application

#### Server

To start the server, run the following in the `/server` directory:

```shell
cargo run
```
