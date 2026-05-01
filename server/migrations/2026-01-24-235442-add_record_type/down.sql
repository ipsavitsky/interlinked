ALTER TABLE records RENAME COLUMN payload TO redirect_url;
ALTER TABLE records DROP COLUMN record_type
