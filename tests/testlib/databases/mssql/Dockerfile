FROM mcr.microsoft.com/mssql/server:latest

USER root
RUN mkdir /init
COPY ./*.sql /init/
COPY ./*.sh /init/
RUN chmod uga+r /init
RUN chmod uga+r /init/*
RUN chmod uga+x /init/*.sh
USER mssql

ENV ACCEPT_EULA=Y 
ENV MSSQL_PID=Express

cmd /init/boot.sh

