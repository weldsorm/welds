use crate::get_conn;
use welds::exts::VecRowExt;
use welds::{WeldsError, WeldsModel};

#[derive(Debug, Clone, WeldsModel)]
#[welds(schema = "welds", table = "Teams")]
#[welds(HasMany(players, Player, "team_id"))]
pub struct Team {
    #[welds(primary_key)]
    pub id: i32,
    pub city_id: i32,
    pub name: String,
}
#[derive(Debug, Clone, WeldsModel)]
#[welds(schema = "welds", table = "Players")]
#[welds(BelongsTo(team, Team, "team_id"))]
pub struct Player {
    #[welds(primary_key)]
    pub id: i32,
    pub team_id: i32,
    pub name: String,
}

#[derive(Debug, PartialEq, WeldsModel)]
pub struct TeamWithPlayerCount {
    pub team_id: i32,
    pub team_name: String,
    pub player_count: i32,
}

#[derive(Debug, PartialEq, WeldsModel)]
pub struct TeamWithLatestPlayer {
    pub team_id: i32,
    pub player_id: i32,
}

#[tokio::test]
async fn should_join_data_with_group_by_and_count() {
    let conn = get_conn().await;

    let query = Team::all()
        .select_as(|t| t.id, "team_id")
        .select_as(|t| t.name, "team_name")
        .left_join(
            |t| t.players,
            Player::all().select_count(|p| p.id, "player_count"),
        )
        .group_by(|t| t.id)
        .group_by(|t| t.name);

    let collection: Vec<TeamWithPlayerCount> =
        query.run(&conn).await.unwrap().collect_into().unwrap();

    assert_eq!(
        collection[0],
        TeamWithPlayerCount {
            team_id: 1,
            team_name: "Liverpool FC".to_string(),
            player_count: 1
        }
    );
    assert_eq!(
        collection[1],
        TeamWithPlayerCount {
            team_id: 2,
            team_name: "Manchester City".to_string(),
            player_count: 1
        }
    );
    assert_eq!(
        collection[2],
        TeamWithPlayerCount {
            team_id: 3,
            team_name: "Manchester United".to_string(),
            player_count: 2
        }
    );
}

#[tokio::test]
async fn should_join_data_with_group_by_and_max() {
    let conn = get_conn().await;

    let query = Team::all()
        .select_as(|t| t.id, "team_id")
        .left_join(
            |t| t.players,
            Player::all().select_max(|p| p.id, "player_id"),
        )
        .group_by(|t| t.id);

    let collection: Vec<TeamWithLatestPlayer> =
        query.run(&conn).await.unwrap().collect_into().unwrap();

    assert_eq!(
        collection[0],
        TeamWithLatestPlayer {
            team_id: 1,
            player_id: 1
        }
    );
    assert_eq!(
        collection[1],
        TeamWithLatestPlayer {
            team_id: 2,
            player_id: 2
        }
    );
    assert_eq!(
        collection[2],
        TeamWithLatestPlayer {
            team_id: 3,
            player_id: 4
        }
    );
}

#[tokio::test]
async fn should_allow_simple_aggregate_functions_without_other_selects() {
    let conn = get_conn().await;

    let result = Team::all().select_max(|t| t.id, "max_id").run(&conn).await;

    assert!(result.is_ok())
}

#[tokio::test]
async fn should_return_an_error_if_group_by_clause_is_required() {
    let conn = get_conn().await;

    let result = Team::all()
        .select(|t| t.name)
        .select_max(|t| t.id, "max_id")
        .run(&conn)
        .await;

    match result {
        Ok(_) => panic!(),
        Err(e) => {
            assert_eq!(
                e.to_string(),
                WeldsError::ColumnMissingFromGroupBy.to_string()
            )
        }
    }
}
