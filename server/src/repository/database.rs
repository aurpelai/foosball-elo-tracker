use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager, Pool},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;

pub struct Database {
    pub pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

impl Database {
    pub fn establish_connection() -> ConnectionManager<PgConnection> {
        dotenv().ok();
        let url = std::env::var("DATABASE_URL")
            .expect("Environment variable 'DATABASE_URL' must be set!");
        ConnectionManager::<PgConnection>::new(url)
    }

    pub fn get_connection_pool(
        manager: ConnectionManager<PgConnection>,
    ) -> Pool<ConnectionManager<PgConnection>> {
        Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool")
    }

    pub fn new() -> Database {
        let connection_manager = Self::establish_connection();
        let connection_pool = Self::get_connection_pool(connection_manager);

        connection_pool
            .get()
            .unwrap()
            .run_pending_migrations(MIGRATIONS)
            .unwrap();

        Database {
            pool: connection_pool,
        }
    }
}
