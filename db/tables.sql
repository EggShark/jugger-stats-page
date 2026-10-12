-- Create tables and table scheemas will also drop them
DROP TABLE IF EXISTS users;

CREATE TABLE users (
    username VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    description VARCHAR,
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY
);
