CREATE UNIQUE INDEX team_index ON teams (
  LEAST (player_one_id, player_two_id),
  GREATEST (player_one_id, player_two_id)
);
