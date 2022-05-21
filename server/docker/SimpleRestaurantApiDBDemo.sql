DROP TABLE orders;

CREATE TABLE orders (
  id SERIAL PRIMARY KEY,
  table_id INTEGER NOT NULL,
  created_at INTEGER NOT NULL,
  item VARCHAR(255) NOT NULL,
  duration int NOT NULL
);