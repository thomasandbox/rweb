-- Your SQL goes here

CREATE TABLE todos (
  id SERIAL NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  status TEXT NOT NULL
);