-- Your SQL goes here
CREATE TABLE brands (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  image_url VARCHAR NOT NULL,
  info VARCHAR NOT NULL
)