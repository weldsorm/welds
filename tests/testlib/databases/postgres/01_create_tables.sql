-- This is a collection of tables used to test Welds.
-- They use a variety of styles and types to test Welds in many ways
-- This collection of table focues on the build in types 


CREATE SCHEMA alt;

CREATE TYPE alt.Color AS ENUM ('Red', 'Green', 'Blue', 'Yellow');

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
  product_id INTEGER REFERENCES products (product_id),
  quantity smallint,
  code text,
  "SoldFor" FLOAT8
);


CREATE TABLE alt.Others (
  id serial PRIMARY KEY,
  interval INTERVAL,
  range_int4 INT4RANGE,
  range_int8 INT8RANGE,
  colour alt.Color
);


CREATE TABLE Thing1 ( id serial PRIMARY KEY, value text);
CREATE TABLE Thing2 ( id serial PRIMARY KEY, value text);
CREATE TABLE Thing3 ( id serial PRIMARY KEY, value text);
CREATE TABLE Thing4 ( id serial PRIMARY KEY, value text);
CREATE TABLE Thing5 ( id serial PRIMARY KEY, value text);
CREATE TABLE Thing6 ( id serial PRIMARY KEY, value text);
CREATE TABLE Thing7 ( id serial PRIMARY KEY, value text);
CREATE TABLE Thing8 ( id serial PRIMARY KEY, value text);
CREATE TABLE Thing9 ( id serial PRIMARY KEY, value text);
