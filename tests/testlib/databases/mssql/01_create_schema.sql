

CREATE SCHEMA welds;
GO

CREATE TABLE welds.Products (
    ID int NOT NULL PRIMARY KEY,
    name varchar(50) NOT NULL,
    Description varchar(max),
    price1 REAL,
    price2 FLOAT(8),
    active bit
);

CREATE TABLE welds.Persons (
    PersonID int,
    LastName varchar(255),
    FirstName varchar(255),
    Address varchar(255),
    City varchar(255)
);

CREATE TABLE welds.Persons2 (
    PersonID int,
    LastName varchar(255),
    FirstName varchar(255),
    Address varchar(255),
    City varchar(255)
);

