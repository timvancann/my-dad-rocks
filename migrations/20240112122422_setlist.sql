CREATE TABLE setlists (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  songs integer[] NOT NULL DEFAULT '{}',
  is_locked BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO setlists (title, songs) VALUES ('Oefenen', array [1, 3, 15, 23]);
