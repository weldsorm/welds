#!/bin/bash


echo "*****************************************************"
echo "booting and seeding data"
echo "*****************************************************"

/init/build.sh &

/opt/mssql/bin/sqlservr
