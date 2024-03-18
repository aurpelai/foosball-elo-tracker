CREATE TABLE
  matches (
    id SERIAL PRIMARY KEY,
    winning_team_id INT NOT NULL REFERENCES teams (id),
    losing_team_id INT NOT NULL REFERENCES teams (id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP,
    winning_team_score SMALLINT,
    losing_team_score SMALLINT,
    CONSTRAINT both_scores_must_either_exist_or_be_null CHECK (
      (
        (winning_team_score IS NOT NULL)
        AND (losing_team_score IS NOT NULL)
        AND (losing_team_score >= 0)
        AND (winning_team_score > losing_team_score)
      )
      OR (
        (winning_team_score IS NULL)
        AND (losing_team_score IS NULL)
      )
    )
  );

SELECT
  diesel_manage_updated_at ('matches');
