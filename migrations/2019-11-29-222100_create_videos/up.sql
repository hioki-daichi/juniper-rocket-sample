CREATE TABLE videos (
  id BIGSERIAL NOT NULL,
  src VARCHAR NOT NULL,
  timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  PRIMARY KEY (id)
);
