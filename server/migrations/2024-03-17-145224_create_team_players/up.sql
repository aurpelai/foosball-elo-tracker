CREATE TABLE
    team_players (
        team_id INT NOT NULL REFERENCES teams (id),
        player_id INT NOT NULL REFERENCES players (id),
        PRIMARY KEY (team_id, player_id)
    );

CREATE OR REPLACE FUNCTION limit_team_size() RETURNS trigger AS $$
DECLARE
    max_player_count INTEGER := 2;
    player_count INTEGER := 0;
    must_check BOOLEAN := false;
BEGIN
    IF TG_OP = 'INSERT' THEN
        must_check := true;
    END IF;

    IF TG_OP = 'UPDATE' THEN
        IF (NEW.team_id != OLD.team_id) THEN
            must_check := true;
        END IF;
    END IF;

    IF must_check THEN
        -- prevent concurrent inserts from multiple transactions
        LOCK TABLE team_players IN EXCLUSIVE MODE;

        SELECT INTO player_count COUNT(*)
        FROM team_players
        WHERE team_id = NEW.team_id;

        IF player_count >= max_player_count THEN
            RAISE EXCEPTION 'Cannot insert more than % team_players for each user.', max_player_count;
        END IF;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TRIGGER limit_team_size
    BEFORE INSERT OR UPDATE ON team_players
    FOR EACH ROW EXECUTE PROCEDURE limit_team_size();
