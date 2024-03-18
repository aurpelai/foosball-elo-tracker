ALTER TABLE matches
DROP CONSTRAINT both_scores_must_either_exist_or_be_null;

DROP TABLE matches;
