-- Your SQL goes here
CREATE TABLE todos (
  id INTEGER PRIMARY KEY,
  body VARCHAR NOT NULL,
  completed BOOLEAN NOT NULL,
  list_id INTEGER NOT NULL
)