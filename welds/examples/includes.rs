use welds::prelude::*;

#[derive(Debug, Clone, WeldsModel)]
#[welds(table = "Teams")]
#[welds(HasMany(players, Player, "team_id"))]
#[welds(BelongsTo(city, City, "city_id"))]
pub struct Team {
    #[welds(primary_key)]
    pub id: i32,
    pub city_id: i32,
    pub name: String,
}

#[derive(Debug, WeldsModel)]
#[welds(table = "Players")]
#[welds(BelongsTo(team, Team, "team_id"))]
pub struct Player {
    #[welds(primary_key)]
    pub id: i32,
    pub team_id: i32,
    pub name: String,
}

#[derive(Debug, Clone, WeldsModel)]
#[welds(table = "Cities")]
#[welds(HasMany(teams, Team, "city_id"))]
pub struct City {
    #[welds(primary_key)]
    pub id: i32,
    pub name: String,
}

// Output struct with borrowed objects (Example 1)
#[derive(Debug)]
pub struct TeamWithRelated<'a> {
    team: &'a Team,
    players: Vec<&'a Player>,
    city: &'a City,
}

// Output struct with owned objects (Example 2)
#[derive(Debug)]
pub struct CityWithTeams {
    city: City,
    teams: Vec<Team>,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection_string = "sqlite::memory:";
    let client = welds::connections::connect(connection_string).await?;

    // Build an in memory DB with a schema
    let schema = include_str!("../../tests/testlib/databases/sqlite/01_create_tables.sql");
    client.execute(schema, &[]).await?;

    // Add some test data for teams, players and cities
    create_test_data(&client).await?;

    // Example 1

    // Fetch all teams who's name starts with "L", and include their related players and cities.
    // The associations are Team->HasMany->Player and Team->BelongsTo->City
    let teams_dataset = Team::where_col(|t| t.name.like("L%"))
        .include(|t| t.players)
        .include(|t| t.city)
        .run(&client)
        .await?;

    // Build a collection of TeamWithRelated objects for each Team, with their related
    // sets of Player and City objects. See TeamWithRelated struct definition.
    let collection: Vec<TeamWithRelated> = teams_dataset
        .iter()
        .map(|team_data| TeamWithRelated {
            team: team_data.as_ref(),
            players: team_data.get(|t| t.players),
            city: team_data.get(|t| t.city)[0],
        })
        .collect();

    dbg!(collection);

    // Example 2

    // Fetch all cities and their related teams (City->HasMany->Team)
    let cities_dataset = City::all().include(|c| c.teams).run(&client).await?;

    // Build a Vec of CityWithTeams objects containing each City and it's corresponding Team(s).
    // The output struct owns it's inner data (see CityWithTeams definition) instead of borrowing.
    let collection: Vec<CityWithTeams> = cities_dataset
        .iter()
        .map(|city_data| CityWithTeams {
            city: city_data.clone(),
            teams: city_data.get_owned(|c| c.teams),
        })
        .collect();

    dbg!(collection);

    // Example 3

    // Fetch all players with their related teams (Player->BelongsTo->Team).
    let players_dataset = Player::all().include(|p| p.team).run(&client).await?;

    // Build a simple Vec of tuples, containing each Player with their related Team
    let collection: Vec<(&Player, &Team)> = players_dataset
        .iter()
        .map(|player_data| (player_data.as_ref(), player_data.get(|p| p.team)[0]))
        .collect();

    dbg!(collection);

    Ok(())
}

// Create and save some Team, City, and Player objects
async fn create_test_data(client: &dyn Client) -> Result<(), Box<dyn std::error::Error>> {
    let mut city1 = City::new();
    city1.id = 1;
    city1.name = "Birmingham".to_string();
    city1.save(client).await?;

    let mut city2 = City::new();
    city2.id = 2;
    city2.name = "Manchester".to_string();
    city2.save(client).await?;

    let mut city3 = City::new();
    city3.id = 3;
    city3.name = "Liverpool".to_string();
    city3.save(client).await?;

    let mut team1 = Team::new();
    team1.id = 1;
    team1.city_id = 2;
    team1.name = "Liverpool FC".to_string();
    team1.save(client).await?;

    let mut team2 = Team::new();
    team2.id = 2;
    team2.city_id = 1;
    team2.name = "Manchester United".to_string();
    team2.save(client).await?;

    let mut team3 = Team::new();
    team3.id = 3;
    team3.city_id = 1;
    team3.name = "Manchester City".to_string();
    team3.save(client).await?;

    let mut player1 = Player::new();
    player1.id = 1;
    player1.team_id = 1;
    player1.name = "Bobby Briggs".to_string();
    player1.save(client).await?;

    let mut player2 = Player::new();
    player2.id = 2;
    player2.team_id = 2;
    player2.name = "Gary Garrison".to_string();
    player2.save(client).await?;

    let mut player3 = Player::new();
    player3.id = 3;
    player3.team_id = 2;
    player3.name = "Tommy Two-shoes".to_string();
    player3.save(client).await?;

    Ok(())
}
