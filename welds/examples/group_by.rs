use welds::errors::Result;
use welds::migrations::prelude::*;
use welds::prelude::*;

#[derive(Debug, WeldsModel)]
#[welds(table = "people")]
#[welds(BelongsTo(team, Team, "team_id"))]
pub struct Person {
    #[welds(primary_key)]
    pub id: i32,
    pub team_id: i32,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Debug, WeldsModel)]
#[welds(table = "teams")]
#[welds(HasMany(people, Person, "team_id"))]
pub struct Team {
    #[welds(primary_key)]
    pub id: i32,
    pub color: String,
}

#[derive(Debug, WeldsModel)]
pub struct Counts {
    count: i32,
}

#[derive(Debug, WeldsModel)]
pub struct CountLastnames {
    lastname: String,
    count: i32,
}

#[async_std::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    // Setup a database database with some fake data
    let client = welds::connections::connect("sqlite::memory:").await?;
    up(&client, &[create_teams_table, create_peoples_table]).await?;
    seed(&client).await?;

    let counts: Vec<Counts> = Person::all()
        .select_count(|x| x.firstname, "count")
        .run(&client)
        .await?
        .collect_into()?;

    // There is no group by, should only have one row returning the total counts
    // Total counts should be the total number of people
    assert_eq!(counts.len(), 1);
    assert_eq!(counts[0].count, 1000);
    println!("Total Names: {}", counts[0].count);

    // Run the same query but this time group by the lastname
    let counts: Vec<CountLastnames> = Person::all()
        .select_count(|x| x.id, "count")
        .select(|x| x.lastname)
        .group_by(|x| x.lastname)
        .run(&client)
        .await?
        .collect_into()?;

    // Display the count for each lastname
    for count in counts {
        println!("Lastname Total: {} : {}", count.lastname, count.count);
    }

    Ok(())
}

// ********************************************************************
// Setup junk
// ********************************************************************

fn create_teams_table(_: &TableState) -> Result<MigrationStep> {
    let m = create_table("teams")
        .id(|c| c("id", Type::Int))
        .column(|c| c("color", Type::String));
    Ok(MigrationStep::new("create_teams_table", m))
}

fn create_peoples_table(_: &TableState) -> Result<MigrationStep> {
    let m = create_table("people")
        .id(|c| c("id", Type::Int))
        .column(|c| c("team_id", Type::Int))
        .column(|c| c("firstname", Type::String))
        .column(|c| c("lastname", Type::String));
    Ok(MigrationStep::new("create_peoples_table", m))
}

// A simple migration to setup the peoples table.
async fn seed(client: &dyn Client) -> Result<()> {
    let teams: Vec<_> = (0..100)
        .map(|i| Team {
            id: 0,
            // everyone has new unique firstname
            color: (if i % 2 == 0 { "Green" } else { "blue" }).to_owned(),
        })
        .collect();
    welds::query::insert::bulk_insert(client, &teams).await?;

    let people: Vec<_> = (0..1000)
        .map(|i| Person {
            id: 0,
            team_id: i % 10,
            // everyone has new unique firstname
            firstname: format!("Firstname(#{})", i),
            // repeating last names
            lastname: format!("Lastname(#{})", i % 10),
        })
        .collect();
    welds::query::insert::bulk_insert(client, &people).await?;
    Ok(())
}
