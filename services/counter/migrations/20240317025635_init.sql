-- initial counter table
CREATE TABLE IF NOT EXISTS counts (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  integer_value INT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_created_at_brin ON counts USING BRIN (created_at);