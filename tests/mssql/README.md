
# Microsoft SQL Server tests for welds

## TEST WARNINGS: 
Microsoft SQL Server is not working with the latest kernel

https://github.com/microsoft/mssql-docker/issues/868


These tests boot up a mssql server in a docker image. They will not if are on 6.7 or above.

This appears to be fix, but leaving here until verified everywhere


## TEST NOTES
These tests need to run in a single thread for the migrations :( 
```
cargo test -- --test-threads=1
```
