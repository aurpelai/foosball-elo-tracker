CREATE TABLE
  player_matches (
    id SERIAL PRIMARY KEY,
    player_id INT NOT NULL REFERENCES players (id),
    match_id INT NOT NULL REFERENCES matches (id)
  );
