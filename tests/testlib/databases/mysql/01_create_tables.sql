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
