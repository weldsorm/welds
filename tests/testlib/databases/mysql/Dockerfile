FROM mysql:8

COPY ./*.sql /docker-entrypoint-initdb.d/
run chmod uga+r /docker-entrypoint-initdb.d/*
