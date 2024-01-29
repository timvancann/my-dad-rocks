CREATE TABLE gigs (
  id SERIAL PRIMARY KEY,
  venue VARCHAR(255) NOT NULL,
  date DATE NOT NULL,
  songs integer[] NOT NULL DEFAULT '{}'
);

INSERT INTO gigs (venue, date, songs)  VALUES ('Oosterhout', '2024-05-19', '{}');
