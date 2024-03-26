CREATE TYPE result_type AS ENUM ('SHUTOUT_WIN', 'WIN', 'LOSS', 'SHUTOUT_LOSS');

CREATE TABLE
  matches (
    id SERIAL PRIMARY KEY,
    timestamp TIMESTAMP NOT NULL DEFAULT NOW (),
    update_timestamp TIMESTAMP,
    winning_score SMALLINT,
    losing_score SMALLINT,
    CONSTRAINT both_scores_must_either_exist_or_be_null CHECK (
      (
        (winning_score IS NOT NULL)
        AND (losing_score IS NOT NULL)
        AND (losing_score >= 0)
        AND (winning_score > losing_score)
      )
      OR (
        (winning_score IS NULL)
        AND (losing_score IS NULL)
      )
    )
  );

SELECT
  diesel_manage_update_timestamp ('matches');
