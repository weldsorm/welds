use welds::prelude::*;

// Enabling the `unstable-api` feature exposes some additional querying methods such as
// `group_by()`, `select_count()` and `select_max()` for grouping and aggregating.
// Using these functions can result in queries which compile correctly but return
// errors at runtime (if the syntax is wrong), so use them with care.

// Team model, with HasMany association to Player
#[derive(Debug, Clone, WeldsModel)]
#[welds(table = "Teams")]
#[welds(HasMany(players, Player, "team_id"))]
pub struct Team {
    #[welds(primary_key)]
    pub id: i32,
    pub name: String,
}

// Player model, with BelongsTo association for Team
#[derive(Debug, WeldsModel)]
#[welds(table = "Players")]
#[welds(BelongsTo(team, Team, "team_id"))]
pub struct Player {
    #[welds(primary_key)]
    pub id: i32,
    pub team_id: i32,
    pub name: String,
}

// Struct for collecting results for Team query with a count of it's associated Player(s)
#[derive(Debug, WeldsModel)]
pub struct TeamWithPlayerCount {
    pub id: i32,
    pub name: String,
    pub player_count: i32,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection_string = "sqlite::memory:";
    let client = welds::connections::connect(connection_string).await?;

    // Build an in memory DB with a schema
    let schema = include_str!("../../tests/testlib/databases/sqlite/01_create_tables.sql");
    client.execute(schema, &[]).await?;

    // Populate the DB some example data
    let data = include_str!("../../tests/testlib/databases/sqlite/02_add_test_data.sql");
    client.execute(data, &[]).await?;

    // Example query; select all columns from teams table where team name starts with "L",
    // left join to the players table, select COUNT(players.id) as "player_count",
    // group by team id and order by team name:
    let query = Team::where_col(|team| team.name.like("L%"))
        .select_all()
        .left_join(
            |team| team.players,
            Player::all().select_count(|player| player.id, "player_count"),
        )
        .group_by(|team| team.id)
        .order_by_asc(|team| team.name);

    // Collect the resulting rows into a Vec of TeamWithPlayerCount structs containing
    // each team's id, name, and player_count
    let collection: Vec<TeamWithPlayerCount> = query.run(&client).await?.collect_into()?;

    for row in collection {
        println!("Count: {:?}", row);
    }

    Ok(())
}
