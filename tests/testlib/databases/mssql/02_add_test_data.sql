
SET IDENTITY_INSERT welds.Products ON;
GO

Insert INTO welds.Products (
  ID,
  name,
  Description,
  price1,
  price2,
  active 
) VALUES (
1
, 'horse'
, 'Mammal aproxamantly 700lb, can be riden'
, 1.10
, 10.11
, 1
), (
2
, 'dog'
, 'Mans Best Friend'
, 2.10
, 20.11
, 1
), (
3
, 'cat'
, 'likes lasagna'
, 3.10
, 30.11
, 0
), (
4
, 'cow'
, 'MOOOOOOOO'
, 4.10
, 40.11
, 0
), (
5
, 'goat'
, 'Why Not'
, 5.10
, 50.11
, 1
), (
6
, 'pig'
, 'Sus domesticus'
, 6.10
, 60.11
, 1
);

SET IDENTITY_INSERT welds.Products OFF;
GO


SET IDENTITY_INSERT welds.Orders ON;
GO

Insert INTO welds.Orders (
  id,
  product_id 
) VALUES 
( 1,1 ),
( 2,1 ),
( 3,1 ),
( 4,2 ),
( 5,2 )

SET IDENTITY_INSERT welds.Orders OFF;
GO
