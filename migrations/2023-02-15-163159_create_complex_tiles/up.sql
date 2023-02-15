CREATE TABLE complex_tiles (
  chunk_x BIGINT NOT NULL,
  chunk_y BIGINT NOT NULL,
  x INTEGER NOT NULL,
  y INTEGER NOT NULL,
  metadata BIGINT NOT NULL,
  PRIMARY KEY(chunk_x, chunk_y, x, y)
)
