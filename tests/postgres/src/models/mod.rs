pub mod enums;
pub mod order;
pub mod other;
pub mod product;
pub mod table_with_array;

use welds::WeldsModel;
#[derive(Debug, WeldsModel)]
#[welds(table = "Thing1")]
pub struct Thing1 {
    #[welds(primary_key)]
    pub id: i32,
    pub value: String,
}

#[derive(Debug, WeldsModel)]
#[welds(table = "Thing2")]
pub struct Thing2 {
    #[welds(primary_key)]
    pub id: i32,
    pub value: String,
}

#[derive(Debug, WeldsModel)]
#[welds(table = "StringThing")]
pub struct StringThing {
    #[welds(primary_key)]
    pub id: String,
    pub value: String,
}

#[derive(WeldsModel)]
#[welds(table = "uuid_id_from_db")]
pub struct UuidIdFromDb {
    #[welds(primary_key)]
    pub id: uuid::Uuid,
    pub name: String,
}

#[derive(WeldsModel)]
#[welds(table = "uuid_id_from_dev")]
pub struct UuidIdFromDev {
    #[welds(primary_key)]
    pub id: uuid::Uuid,
    pub name: String,
}

#[derive(WeldsModel)]
#[welds(table = "teams")]
#[welds(HasMany(players, Player, "team_id"))]
pub struct Team {
    #[welds(primary_key)]
    pub id: i32,
    pub name: String
}

#[derive(WeldsModel)]
#[welds(table = "players")]
#[welds(BelongsTo(team, Team, "team_id"))]
pub struct Player {
    #[welds(primary_key)]
    pub id: i32,
    pub team_id: i32,
    pub name: String
}
