
<div align="center">
  <img src="https://raw.githubusercontent.com/weldsorm/welds/main/page/src/assets/images/banner.png"/>
  <h3>An async ORM written in rust using the sqlx framework.</h3>
</div>

# Welds - CLI

Welds is an ORM for Rust. 
This cli is tool to help use welds.
It is used to generate model struct definitions.

# How to use

1) Set a connection string to your database using the ENV DATABASE_URL

```bash
export DATABASE_URL=postgres://postgres:password@localhost:5432
```

2) use the welds-cli to create a `welds.yaml` database definition file.
```bash
welds update
```

3) use the welds-cli to generate rust code. 
```bash
welds generate
```
