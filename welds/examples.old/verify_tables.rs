/// This example uses the mysql testing database in the `tests` directory of welds
/// you can use it for this example easily with the following commands
/// ```bash
/// cd tests/testlib/databases/mysql
/// docker-compose build
/// docker-compose up -d
/// export DATABASE_URL=mysql://root:welds!123@localhost:3306/weldstests
/// ```
use welds::WeldsModel;

#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Mysql))]
#[welds(table = "i_dont_exist")]
pub struct NotInDb {
    #[welds(primary_key)]
    pub id: i32,
}

#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Mysql))]
#[welds(schema = "mysql", table = "Orders")]
pub struct Order {
    #[welds(primary_key)]
    pub id: i32,
    pub product_id: String,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let unknown = welds::connection::AnyPool::connect().await.expect(
        "Expected DATABASE_URL to point to a mysql database. did you boot the test database?",
    );
    let pool = unknown
        .as_mysql()
        .expect("Expected a mysql database.")
        .clone();

    // the NotInDb struct will only report back that is isn't in the database
    println!();
    let diff = welds::check::schema::<NotInDb, _, _>(&pool).await?;
    for d in diff {
        println!("{}", d);
    }

    // Get all the things that are different from the Order struct and the order table in the DB
    let diff = welds::check::schema::<Order, _, _>(&pool).await?;
    for d in &diff {
        println!("{}", d);
    }

    println!();
    // let look over just the columns that have changed types
    // on the Orders table/struct
    println!("Orders Changed Columns:");
    for d in &diff {
        if let Some(changed) = d.kind.as_changed() {
            if changed.type_changed() {
                println!("{}", changed);
            }
        }
    }

    Ok(())
}
