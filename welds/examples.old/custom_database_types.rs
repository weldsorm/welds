use welds::connection::Connection;
/// This example uses the postgres testing database in the `tests` directory of welds
/// you can use it for this example easily with the following commands
/// ```bash
/// cd tests/testlib/databases/postgres
/// docker-compose build
/// docker-compose up -d
/// export DATABASE_URL=postgres://postgres:password@localhost:5432
/// ```
use welds::WeldsModel;

/// Defend the enum that will match up to the custom Database type
///
/// This is just a standard sqlx::Type
///
/// Welds REQUIRES its structs to impl default.
/// You need a default color to be able to add it to your struct
#[derive(sqlx::Type, Debug, Clone, PartialEq, Default)]
#[sqlx(type_name = "Color")]
pub enum Color {
    Red,
    Green,
    #[default]
    Blue,
}

#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Postgres))]
#[welds(table = "balloons")]
pub struct Balloon {
    #[welds(primary_key)]
    pub id: i32,
    pub color: Color,
}

// NOTE: this has already been ran in the database
// It is here so you can see what it would look like
#[allow(dead_code)]
const CREATE_COLOR_TYPE: &str = "
CREATE SCHEMA alt;
CREATE TYPE alt.Color AS ENUM ('Red', 'Green', 'Blue', 'Yellow');
";

const CREATE_TABLE: &str = "
CREATE TABLE balloons (
  id serial PRIMARY KEY,
  color alt.Color
);
";

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let unknown = welds::connection::AnyPool::connect().await.expect(
        "Expected DATABASE_URL to point to a postgres database. did you boot the test database?",
    );
    let pool = unknown.as_postgres().unwrap().clone();
    let trans = pool.begin().await?;

    // Create the balloons table
    trans.execute(CREATE_TABLE, Default::default()).await?;

    //create some balloons in the database
    for _ in 0..99 {
        let mut balloon = Balloon::new();
        balloon.color = Color::Red;
        balloon.save(&trans).await?;
    }

    // check that the database records were created.
    let count = Balloon::all().count(&trans).await?;

    println!("{} red balloons", count);

    trans.rollback().await?;
    Ok(())
}
