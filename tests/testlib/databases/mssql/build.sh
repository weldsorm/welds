#!/bin/bash

sleep 5

cd /init

for f in *.sql; do
  cat $f | /opt/mssql-tools18/bin/sqlcmd -C -S localhost -U sa -P $SA_PASSWORD
done

echo "All SQL Seeded"
