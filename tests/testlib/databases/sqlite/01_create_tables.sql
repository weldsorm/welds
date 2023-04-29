
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
  price REAL,
  product_id INTEGER NOT NULL,
  product_id2 INTEGER,
  code text,
  FOREIGN KEY(product_id) REFERENCES Products(product_id),
  FOREIGN KEY(product_id2) REFERENCES Products(product_id)
);
