CREATE TABLE
  team_results (
    team_id INT NOT NULL REFERENCES teams (id),
    match_id INT NOT NULL REFERENCES matches (id),
    result_type result_type NOT NULL,
    rating SMALLINT NOT NULL,
    rating_delta SMALLINT NOT NULL,
    PRIMARY KEY (team_id, match_id)
  );
