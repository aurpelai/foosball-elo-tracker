use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::player::{NewPlayer, Player};
use crate::models::schema::player::dsl::*;

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

    // Loads all players
    pub fn get_players(&self) -> Vec<Player> {
        player
            .load::<Player>(&mut self.pool.get().unwrap())
            .expect("Failed to get players.")
    }

    // Loads a single player using heir id
    pub fn get_player(&self, player_id: i32) -> Option<Player> {
        player
            .find(player_id)
            .first::<Player>(&mut self.pool.get().unwrap())
            .ok()
    }

    pub fn create_player(&self, new_player: NewPlayer) -> Result<Player, diesel::result::Error> {
        diesel::insert_into(player)
            .values(&new_player)
            .get_result(&mut self.pool.get().unwrap())
    }

    pub fn delete_player(&self, player_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(player.filter(id.eq(player_id))).execute(&mut self.pool.get().unwrap())
    }

    pub fn update_player(&self, new_player: Player) -> Result<Player, diesel::result::Error> {
        diesel::update(player.filter(id.eq(new_player.id)))
            .set(&new_player)
            .get_result(&mut self.pool.get().unwrap())
    }
}
