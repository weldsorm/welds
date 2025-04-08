
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


SET IDENTITY_INSERT welds.Profiles ON;
GO
INSERT INTO welds.Profiles (id, user_id, image_url) VALUES
(1, 1, "cat.jpeg"),
(2, 3, "dog.jpeg"),
(3, 4, "bird.png");
SET IDENTITY_INSERT welds.Profiles OFF;
GO


SET IDENTITY_INSERT welds.Users ON;
GO
INSERT INTO welds.Users (id, name) VALUES
(1, "Alice"),
(2, "Bob"),
(3, "Catherine"),
(4, "Danny");
SET IDENTITY_INSERT welds.Users OFF;
GO


SET IDENTITY_INSERT welds.Cities ON;
GO
INSERT INTO welds.Cities (id, name) VALUES
(1, "Birmingham"),
(2, "Liverpool"),
(3, "Manchester");
SET IDENTITY_INSERT welds.Cities OFF;
GO


SET IDENTITY_INSERT welds.Teams ON;
GO
INSERT INTO welds.Teams (id, city_id, name) VALUES
(1, 2, "Liverpool FC"),
(2, 3, "Manchester City"),
(3, 3, "Manchester United");
SET IDENTITY_INSERT welds.Teams OFF;
GO


SET IDENTITY_INSERT welds.Players ON;
GO
INSERT INTO welds.Players (id, team_id, name) VALUES
(1, 1, "Andy Anderson"),
(2, 2, "Bobby Biggs"),
(3, 3, "Chris Christoferson"),
(4, 3, "Danny Dier");
SET IDENTITY_INSERT welds.Players OFF;
GO
