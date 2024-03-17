use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::player::{NewPlayer, Player};
use crate::models::schema::players::dsl::*;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pub pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let result = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        Database { pool: result }
    }

    pub fn get_players(&self) -> Vec<Player> {
        players
            .load::<Player>(&mut self.pool.get().unwrap())
            .expect("Failed to get players.")
    }

    pub fn get_player(&self, player_id: i32) -> Option<Player> {
        players
            .find(player_id)
            .first::<Player>(&mut self.pool.get().unwrap())
            .ok()
    }

    pub fn create_player(&self, new_player: NewPlayer) -> Result<Player, diesel::result::Error> {
        diesel::insert_into(players)
            .values(&new_player)
            .get_result(&mut self.pool.get().unwrap())
    }

    pub fn delete_player(&self, player_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(players.find(player_id)).execute(&mut self.pool.get().unwrap())
    }

    pub fn update_player(&self, player: Player) -> Result<Player, diesel::result::Error> {
        diesel::update(players.find(player.id))
            .set(&player)
            .get_result(&mut self.pool.get().unwrap())
    }
}
