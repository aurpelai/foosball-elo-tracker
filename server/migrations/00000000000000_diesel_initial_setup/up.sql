-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.




-- Sets up a trigger for the given table to automatically set a column called
-- `update_timestamp` whenever the row is modified (unless `update_timestamp` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, update_timestamp TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_update_timestamp('users');
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_update_timestamp(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_update_timestamp BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_update_timestamp()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_update_timestamp() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.update_timestamp IS NOT DISTINCT FROM OLD.update_timestamp
    ) THEN
        NEW.update_timestamp := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
