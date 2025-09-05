CREATE TABLE IF NOT EXISTS accounts (
  id INT PRIMARY KEY AUTO_INCREMENT,
  account VARCHAR(255) NOT NULL UNIQUE,
  password VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS players (
  id INT PRIMARY KEY AUTO_INCREMENT,
  name VARCHAR(255) NOT NULL UNIQUE,
  level INT NOT NULL,
  vocation VARCHAR(255) NOT NULL,
  account VARCHAR(255) NOT NULL
);

-- Test data for benchmarks
INSERT IGNORE INTO accounts (account, password) VALUES ('test', 'test');
INSERT IGNORE INTO players (name, level, vocation, account) VALUES ('test-Knight', 220, 'Knight', 'test');
