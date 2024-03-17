CREATE TABLE teams (
    id SERIAL PRIMARY KEY,
    player_one_id INT NOT NULL REFERENCES players(id),
    player_two_id INT NOT NULL REFERENCES players(id)
);
