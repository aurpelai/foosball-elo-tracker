CREATE TABLE
  player_results (
    player_id INT NOT NULL REFERENCES players (id),
    match_id INT NOT NULL REFERENCES matches (id),
    result_type result_type NOT NULL,
    rating SMALLINT NOT NULL,
    rating_delta SMALLINT NOT NULL,
    PRIMARY KEY (player_id, match_id)
  );
