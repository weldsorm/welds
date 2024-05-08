

CREATE SCHEMA welds;
GO

CREATE TABLE _welds_migrations(
  id BIGINT NOT NULL IDENTITY PRIMARY KEY,
  name VARCHAR ( 512 ) NOT NULL,
  when_applied BIGINT NOT NULL,
  rollback_sql TEXT NOT NULL,
);

CREATE TABLE welds.Products (
    ID INT NOT NULL IDENTITY PRIMARY KEY,
    name varchar(50) NOT NULL,
    Description varchar(max),
    price1 REAL,
    price2 FLOAT(8),
    active bit
);

CREATE TABLE welds.Orders (
    id INT NOT NULL IDENTITY PRIMARY KEY,
    product_id int NOT NULL FOREIGN KEY (product_id) REFERENCES welds.Products(ID),
    product_id2 int FOREIGN KEY (product_id2) REFERENCES welds.Products(ID),
    code varchar(max),
);

CREATE TABLE welds.Persons (
    PersonID INT IDENTITY PRIMARY KEY,
    LastName varchar(255),
    FirstName varchar(255),
    Address varchar(255),
    City varchar(255)
);

CREATE TABLE welds.Persons2 (
    PersonID INT IDENTITY PRIMARY KEY,
    LastName varchar(255),
    FirstName varchar(255),
    Address varchar(255),
    City varchar(255)
);

CREATE TABLE welds.Thing1 ( id INT IDENTITY PRIMARY KEY, value varchar(max) );
CREATE TABLE welds.Thing2 ( id INT IDENTITY PRIMARY KEY, value varchar(max) );
CREATE TABLE welds.Thing3 ( id INT IDENTITY PRIMARY KEY, value varchar(max) );
CREATE TABLE welds.Thing4 ( id INT IDENTITY PRIMARY KEY, value varchar(max) );
CREATE TABLE welds.Thing5 ( id INT IDENTITY PRIMARY KEY, value varchar(max) );
CREATE TABLE welds.Thing6 ( id INT IDENTITY PRIMARY KEY, value varchar(max) );
CREATE TABLE welds.Thing7 ( id INT IDENTITY PRIMARY KEY, value varchar(max) );
CREATE TABLE welds.Thing8 ( id INT IDENTITY PRIMARY KEY, value varchar(max) );
CREATE TABLE welds.Thing9 ( id INT IDENTITY PRIMARY KEY, value varchar(max) );

CREATE TABLE welds.StringThing ( id VARCHAR(64) PRIMARY KEY, value varchar(max) );

