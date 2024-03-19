CREATE TABLE
  team_matches (
    id SERIAL PRIMARY KEY,
    team_id INT NOT NULL REFERENCES teams (id),
    match_id INT NOT NULL REFERENCES matches (id)
  );
