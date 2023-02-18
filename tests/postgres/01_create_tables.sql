-- This is a collection of tables used to test Welds.
-- They use a variety of styles and types to test Welds in many ways
-- This collection of table focues on the build in types 

CREATE TABLE Products (
  product_id serial PRIMARY KEY,
  name VARCHAR ( 50 ) UNIQUE NOT NULL,
  Description text,
  price1 REAL,
  price2 FLOAT8,
  price3 MONEY,
  barcode BYTEA,
  active bool
);

CREATE TABLE Orders (
  id bigserial PRIMARY KEY,
  product_id int,
  quantity smallint,
  code char,
  SoldFor MONEY
);

CREATE TABLE Others (
  id serial PRIMARY KEY,
  interval INTERVAL,
  range_int4 INT4RANGE,
  range_int8 INT8RANGE
);


