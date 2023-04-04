
CREATE TABLE Products (
  product_id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  description text,
  price1 REAL,
  price2 REAL,
  active bool
);

CREATE TABLE Orders (
  id INTEGER PRIMARY KEY,
  product_id INTEGER NOT NULL,
  price REAL
);
