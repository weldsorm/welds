-- This is a collection of tables used to test Welds.
-- They use a variety of styles and types to test Welds in many ways
-- This collection of table focues on the build in types 

CREATE DATABASE weldstests;
USE weldstests;

CREATE TABLE Products (
  product_id INT NOT NULL AUTO_INCREMENT,
  name VARCHAR ( 50 ) UNIQUE NOT NULL,
  description text,
  price1 FLOAT,
  price2 DOUBLE,
  active bool,
  PRIMARY KEY (product_id)
);
