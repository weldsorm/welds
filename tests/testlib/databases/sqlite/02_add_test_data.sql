
Insert INTO Products (
  product_id,
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
, true
), (
2
, 'dog'
, 'Mans Best Friend'
, 2.10
, 20.11
, true
), (
3
, 'cat'
, 'likes lasagna'
, 3.10
, 30.11
, false
), (
4
, 'cow'
, 'MOOOOOOOO'
, 4.10
, 40.11
, false
), (
5
, 'goat'
, 'Why Not'
, 5.10
, 50.11
, true
), (
6
, 'pig'
, 'Sus domesticus'
, 6.10
, 60.11
, true
);



Insert INTO Orders (
  id,
  price,
  product_id,
  product_id2,
  code
) VALUES (
1
, 11.11
, 1
, null
, ''
), (
2
, 22.22
, 1
, null
, ''
), (
3
, 33.33
, 2
, null
, ''
);

INSERT INTO Profiles (id, image_url) VALUES
(1, "cat.jpeg"),
(2, "dog.jpeg"),
(3, "bird.png");

INSERT INTO Users (id, profile_id, name) VALUES
(1, 1, "Alice"),
(2, NULL, "Bob"),
(3, 2, "Catherine"),
(4, 3, "Danny");

INSERT INTO Cities (id, name) VALUES
(1, "Birmingham"),
(2, "Liverpool"),
(3, "Manchester");

INSERT INTO Teams (id, city_id, name) VALUES
(1, 2, "Liverpool FC"),
(2, 3, "Manchester City"),
(3, 3, "Manchester United");

INSERT INTO Players (id, team_id, name) VALUES
(1, 1, "Andy Anderson"),
(2, 2, "Bobby Biggs"),
(3, 3, "Chris Christoferson"),
(4, 3, "Danny Dier");
