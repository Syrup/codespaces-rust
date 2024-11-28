CREATE TABLE IF NOT EXISTS todos (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title VARCHAR(255) NOT NULL,
  description TEXT NOT NULL,
  done BOOLEAN NOT NULL DEFAULT FALSE
);