-- Add migration script here
CREATE TYPE ticket_status AS ENUM (
    'open',
    'inprogress',
    'resolved'
);

ALTER TABLE tickets
ADD COLUMN status ticket_status NOT NULL DEFAULT 'open';