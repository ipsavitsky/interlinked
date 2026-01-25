ALTER TABLE records ADD COLUMN record_type TEXT NOT NULL DEFAULT 'link' CHECK (
    record_type IN ('link', 'note')
);
ALTER TABLE records RENAME COLUMN redirect_url TO payload
