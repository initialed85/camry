CREATE DATABASE camry;

ALTER DATABASE camry
SET
    SEARCH_PATH TO public;

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    stream (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        name TEXT NOT NULL,
        url TEXT NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
        updated_at TIMESTAMPTZ NULL,
        deleted_at TIMESTAMPTZ NULL
    );
