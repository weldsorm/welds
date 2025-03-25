use super::get_conn;
use welds::prelude::*;
use welds::exts::VecRowExt;

#[derive(Debug, Clone, WeldsModel)]
#[welds(table = "teams")]
#[welds(HasMany(players, Player, "team_id"))]
pub struct Team {
    #[welds(primary_key)]
    pub id: i32,
    pub city_id: i32,
    pub name: String,
}
#[derive(Debug, Clone, WeldsModel)]
#[welds(table = "players")]
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
    pub player_count: i64,
}

#[derive(Debug, PartialEq, WeldsModel)]
pub struct TeamWithLatestPlayer {
    pub team_id: i32,
    pub player_id: i32,
}

#[test]
fn should_join_data_with_group_by_and_count() {
    async_std::task::block_on(async {
        let conn = get_conn().await;

        let query = Team::all()
            .select_as(|t| t.id, "team_id")
            .select_as(|t| t.name, "team_name")
            .left_join(|t| t.players,
                Player::all().select_count(|p| p.id, "player_count")
            )
            .group_by(|t| t.id)
            .order_by_asc(|t| t.id);

        let collection: Vec<TeamWithPlayerCount> = query.run(&conn).await.unwrap().collect_into().unwrap();

        assert_eq!(
            collection[0],
            TeamWithPlayerCount { team_id: 1, team_name: "Liverpool FC".to_string(), player_count: 1 }
        );
        assert_eq!(
            collection[1],
            TeamWithPlayerCount { team_id: 2, team_name: "Manchester City".to_string(), player_count: 1 }
        );
        assert_eq!(
            collection[2],
            TeamWithPlayerCount { team_id: 3, team_name: "Manchester United".to_string(), player_count: 2 }
        );
    })
}

#[test]
fn should_join_data_with_group_by_and_max() {
    async_std::task::block_on(async {
        let conn = get_conn().await;

        let query = Team::all()
            .select_as(|t| t.id, "team_id")
            .left_join(|t| t.players,
                Player::all().select_max(|p| p.id, "player_id")
            )
            .group_by(|t| t.id)
            .order_by_asc(|t| t.id);

        let collection: Vec<TeamWithLatestPlayer> = query.run(&conn).await.unwrap().collect_into().unwrap();

        assert_eq!(
            collection[0],
            TeamWithLatestPlayer { team_id: 1, player_id: 1 }
        );
        assert_eq!(
            collection[1],
            TeamWithLatestPlayer { team_id: 2, player_id: 2 }
        );
        assert_eq!(
            collection[2],
            TeamWithLatestPlayer { team_id: 3, player_id: 4 }
        );
    })
}
