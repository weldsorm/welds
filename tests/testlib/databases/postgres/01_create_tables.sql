-- This is a collection of tables used to test Welds.
-- They use a variety of styles and types to test Welds in many ways
-- This collection of table focues on the build in types 
CREATE EXTENSION pgcrypto;


CREATE SCHEMA alt;

-- SQLX only support custom type on public :(
CREATE TYPE public.Color AS ENUM ('Red', 'Green', 'Blue', 'Yellow');

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
  colour public.Color
);

CREATE TABLE BadColumnNames (
  " id" bigserial PRIMARY KEY,
  "camelCase" text,
  "col With     SPACES" text,
  "col With -- DASH" text,
  "select" text,
  "from 'quotes' ed" text
);

CREATE TABLE uuid_id_from_db (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(128) NOT NULL
);

CREATE TABLE uuid_id_from_dev (
    id UUID PRIMARY KEY,
    name VARCHAR(128) NOT NULL
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

CREATE TABLE StringThing ( id text PRIMARY KEY, value text NOT NULL );

CREATE TABLE alt.table_with_arrays (
  id serial PRIMARY KEY,
  numbers int[]
);

CREATE TABLE extra_types (
    id UUID PRIMARY KEY,
    json_col JSONB NOT NULL,
    date_col DATE NOT NULL,
    time_col TIME NOT NULL,
    datetime_col TIMESTAMP NOT NULL,
    datetimetz_col TIMESTAMPTZ NOT NULL
);

CREATE TABLE teams (
    id serial PRIMARY KEY,
    city_id INTEGER NOT NULL,
    name text
);

CREATE TABLE players (
    id serial PRIMARY KEY,
    team_id INTEGER NOT NULL,
    name text
);
