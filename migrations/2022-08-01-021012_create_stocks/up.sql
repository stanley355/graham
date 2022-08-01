-- Your SQL goes here
CREATE TABLE stocks (
  id SERIAL PRIMARY KEY,
  code VARCHAR NOT NULL,
  name VARCHAR NOT NULL
);