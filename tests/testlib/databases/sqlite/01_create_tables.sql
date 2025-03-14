
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

CREATE TABLE Thing1 ( id INTEGER PRIMARY KEY, value text NOT NULL );
CREATE TABLE Thing2 ( id INTEGER PRIMARY KEY, value text NOT NULL );
CREATE TABLE Thing3 ( id INTEGER PRIMARY KEY, value text NOT NULL );
CREATE TABLE Thing4 ( id INTEGER PRIMARY KEY, value text NOT NULL );
CREATE TABLE Thing5 ( id INTEGER PRIMARY KEY, value text NOT NULL );
CREATE TABLE Thing6 ( id INTEGER PRIMARY KEY, value text NOT NULL );
CREATE TABLE Thing7 ( id INTEGER PRIMARY KEY, value text NOT NULL );
CREATE TABLE Thing8 ( id INTEGER PRIMARY KEY, value text NOT NULL );
CREATE TABLE Thing9 ( id INTEGER PRIMARY KEY, value text NOT NULL );

CREATE TABLE StringThing ( id text PRIMARY KEY, value text NOT NULL );

CREATE TABLE extra_types (
    id TEXT PRIMARY KEY,
    json_col TEXT NOT NULL,
    date_col DATE NOT NULL,
    time_col TIME NOT NULL,
    datetime_col DATETIME NOT NULL,
    datetimetz_col DATETIME NOT NULL
);

CREATE TABLE Users (
    id INTEGER PRIMARY KEY,
    profile_id INTEGER,
    name TEXT
);

CREATE TABLE Profiles (
    id INTEGER PRIMARY KEY,
    image_url TEXT
);

CREATE TABLE Teams (
    id INTEGER PRIMARY KEY,
    city_id INTEGER NOT NULL,
    name TEXT
);

CREATE TABLE Players (
    id INTEGER PRIMARY KEY,
    team_id INTEGER NOT NULL,
    name TEXT
);

CREATE TABLE Cities (
    id INTEGER PRIMARY KEY,
    name TEXT
);
