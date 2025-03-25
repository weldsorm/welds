

Insert INTO Products (
  product_id,
  name,
  Description,
  price1,
  price2,
  price3,
  barcode,
  active 
) VALUES (
1
, 'horse'
, 'Mammal aproxamantly 700lb, can be riden'
, 1.10
, 10.11
, 100.12
, 'HORSE-MODEL-1'
, true
), (
2
, 'dog'
, 'Mans Best Friend'
, 2.10
, 20.11
, 200.12
, 'DOGGO-MODEL-1'
, true
), (
3
, 'cat'
, 'likes lasagna'
, 3.10
, 30.11
, 300.12
, 'GARFIELD-1'
, false
), (
4
, 'cow'
, 'MOOOOOOOO'
, 4.10
, 40.11
, 400.12
, 'MOO-1'
, false
), (
5
, 'goat'
, 'Why Not'
, 5.10
, 50.11
, 500.12
, 'GOAT-1'
, true
), (
6
, 'pig'
, 'Sus domesticus'
, 6.10
, 60.11
, 600.12
, 'CHRIS-B-BAKING'
, true
);


INSERT INTO Orders (
  id,
  product_id,
  quantity,
  code,
  "SoldFor"
) VALUES (
1
, 2
, 101
, 'D'
, 0.01
), (
2
, 1
, 1
, 'H'
, 999.999
), (
3
, 1
, 6
, 'C'
, 1000000.09
);

INSERT INTO teams (id, city_id, name) VALUES
(1, 2, 'Liverpool FC'),
(2, 3, 'Manchester City'),
(3, 3, 'Manchester United');

INSERT INTO players (id, team_id, name) VALUES
(1, 1, 'Andy Anderson'),
(2, 2, 'Bobby Biggs'),
(3, 3, 'Chris Christoferson'),
(4, 3, 'Danny Dier');


-- RESET THE NEXT IDs
SELECT setval('products_product_id_seq', COALESCE((SELECT MAX(product_id)+1 FROM products), 1), false);
SELECT setval('orders_id_seq', COALESCE((SELECT MAX(id)+1 FROM orders), 1), false);
