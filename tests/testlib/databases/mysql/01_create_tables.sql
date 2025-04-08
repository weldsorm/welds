-- This is a collection of tables used to test Welds.
-- They use a variety of styles and types to test Welds in many ways
-- This collection of table focues on the build in types 

CREATE DATABASE weldstests;
USE weldstests;

CREATE TABLE _welds_migrations(
  id BIGINT NOT NULL AUTO_INCREMENT,
  name VARCHAR ( 512 ) UNIQUE NOT NULL,
  when_applied BIGINT NOT NULL,
  rollback_sql TEXT NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE Products (
  product_id INT NOT NULL AUTO_INCREMENT,
  name VARCHAR ( 50 ) UNIQUE NOT NULL,
  description text,
  price1 FLOAT,
  price2 DOUBLE,
  active bool,
  PRIMARY KEY (product_id)
);



CREATE TABLE Thing1 ( id INT NOT NULL AUTO_INCREMENT, value text, PRIMARY KEY(id) );
CREATE TABLE Thing2 ( id INT NOT NULL AUTO_INCREMENT, value text, PRIMARY KEY(id) );
CREATE TABLE Thing3 ( id INT NOT NULL AUTO_INCREMENT, value text, PRIMARY KEY(id) );
CREATE TABLE Thing4 ( id INT NOT NULL AUTO_INCREMENT, value text, PRIMARY KEY(id) );
CREATE TABLE Thing5 ( id INT NOT NULL AUTO_INCREMENT, value text, PRIMARY KEY(id) );
CREATE TABLE Thing6 ( id INT NOT NULL AUTO_INCREMENT, value text, PRIMARY KEY(id) );
CREATE TABLE Thing7 ( id INT NOT NULL AUTO_INCREMENT, value text, PRIMARY KEY(id) );
CREATE TABLE Thing8 ( id INT NOT NULL AUTO_INCREMENT, value text, PRIMARY KEY(id) );
CREATE TABLE Thing9 ( id INT NOT NULL AUTO_INCREMENT, value text, PRIMARY KEY(id) );

CREATE TABLE StringThing ( id VARCHAR(64) NOT NULL, value text NOT NULL, PRIMARY KEY(id) );


CREATE TABLE extra_types (
    id VARCHAR(36) PRIMARY KEY,
    json_col JSON NOT NULL,
    date_col DATE NOT NULL,
    time_col TIME NOT NULL,
    datetime_col DATETIME NOT NULL,
    datetimetz_col TIMESTAMP NOT NULL
);

CREATE TABLE Users (
  	id INT NOT NULL AUTO_INCREMENT,
    name TEXT,
	  PRIMARY KEY(id)
);

CREATE TABLE Profiles (
  	id INT NOT NULL AUTO_INCREMENT,
    user_id INT NOT NULL,
    image_url TEXT,
	  PRIMARY KEY(id)
);

CREATE TABLE Teams (
  	id INT NOT NULL AUTO_INCREMENT,
    city_id INT NOT NULL,
    name TEXT,
	  PRIMARY KEY(id)
);

CREATE TABLE Players (
  	id INT NOT NULL AUTO_INCREMENT,
    team_id INT NOT NULL,
    name TEXT,
	  PRIMARY KEY(id)
);

CREATE TABLE Cities (
  	id INT NOT NULL AUTO_INCREMENT,
    name TEXT,
	  PRIMARY KEY(id)
);


USE mysql;
CREATE TABLE Orders (
  id INT NOT NULL AUTO_INCREMENT,
  product_id INT,
  product_id2 INT,
  price FLOAT,
  code VARCHAR ( 50 ) NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (product_id)  REFERENCES weldstests.Products(product_id),
  FOREIGN KEY (product_id2) REFERENCES weldstests.Products(product_id)
);

