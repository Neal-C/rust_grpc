
CREATE TABLE IF NOT EXISTS quote (
    id UUID PRIMARY KEY,
    book VARCHAR(255) NOT NULL,
    quote VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    UNIQUE(book, quote)
);