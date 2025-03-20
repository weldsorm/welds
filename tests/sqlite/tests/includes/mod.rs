use crate::get_conn;
use welds::WeldsModel;

#[derive(Debug, WeldsModel)]
#[welds(table = "Users")]
#[welds(HasOne(profile, Profile, "profile_id"))]
pub struct User {
    #[welds(primary_key)]
    pub id: i32,
    pub profile_id: Option<i32>,
    pub name: String,
}
#[derive(Debug, WeldsModel)]
#[welds(table = "Profiles")]
#[welds(BelongsToOne(user, User, "profile_id"))]
pub struct Profile {
    #[welds(primary_key)]
    pub id: i32,
    pub image_url: String,
}
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
#[derive(Debug, Clone, WeldsModel)]
#[welds(table = "Players")]
#[welds(BelongsTo(team, Team, "team_id"))]
pub struct Player {
    #[welds(primary_key)]
    pub id: i32,
    pub team_id: i32,
    pub name: String,
}
#[derive(Debug, WeldsModel)]
#[welds(table = "Cities")]
#[welds(HasMany(teams, Team, "city_id"))]
pub struct City {
    #[welds(primary_key)]
    pub id: i32,
    pub name: String,
}

#[test]
fn should_load_included_with_has_one() {
    async_std::task::block_on(async {
        let conn = get_conn().await;

        let dataset = User::all().include(|x| x.profile).run(&conn).await.unwrap();

        let output = dataset
            .iter()
            .map(|data| {
                (
                    data.id,
                    data.get(|x| x.profile)
                        .into_iter()
                        .map(|x| x.id)
                        .collect::<Vec<i32>>(),
                )
            })
            .collect::<Vec<(i32, Vec<i32>)>>();

        let expected = vec![
            (1, vec![1]),
            (2, vec![]), // Profile is optional and User 2 has no profile (None/NULL)
            (3, vec![2]),
            (4, vec![3]),
        ];

        assert_eq!(expected, output);
    })
}

#[test]
fn should_load_included_with_belongs_to_one() {
    async_std::task::block_on(async {
        let conn = get_conn().await;

        let dataset = Profile::all().include(|x| x.user).run(&conn).await.unwrap();

        let output = dataset
            .iter()
            .map(|data| {
                (
                    data.id,
                    data.get(|x| x.user)
                        .into_iter()
                        .map(|x| x.id)
                        .collect::<Vec<i32>>(),
                )
            })
            .collect::<Vec<(i32, Vec<i32>)>>();

        let expected = vec![(1, vec![1]), (2, vec![3]), (3, vec![4])];

        assert_eq!(expected, output);
    })
}

#[test]
fn should_load_included_with_has_many() {
    async_std::task::block_on(async {
        let conn = get_conn().await;

        let dataset = Team::all().include(|x| x.players).run(&conn).await.unwrap();

        let output = dataset
            .iter()
            .map(|data| {
                (
                    data.id,
                    data.get(|x| x.players)
                        .into_iter()
                        .map(|x| x.id)
                        .collect::<Vec<i32>>(),
                )
            })
            .collect::<Vec<(i32, Vec<i32>)>>();

        let expected = vec![(1, vec![1]), (2, vec![2]), (3, vec![3, 4])];

        assert_eq!(expected, output);
    })
}

#[test]
fn should_load_included_with_belongs_to() {
    async_std::task::block_on(async {
        let conn = get_conn().await;

        let dataset = Team::all().include(|x| x.city).run(&conn).await.unwrap();

        let output = dataset
            .iter()
            .map(|data| {
                (
                    data.id,
                    data.get(|x| x.city)
                        .into_iter()
                        .map(|x| x.id)
                        .collect::<Vec<i32>>(),
                )
            })
            .collect::<Vec<(i32, Vec<i32>)>>();

        let expected = vec![(1, vec![2]), (2, vec![3]), (3, vec![3])];

        assert_eq!(expected, output);
    })
}

#[test]
fn should_load_included_with_multiple_associations() {
    async_std::task::block_on(async {
        let conn = get_conn().await;

        let dataset = Team::all()
            .include(|x| x.players)
            .include(|x| x.city)
            .run(&conn)
            .await
            .unwrap();

        let output = dataset
            .iter()
            .map(|data| {
                (
                    data.id,
                    data.get(|x| x.players)
                        .into_iter()
                        .map(|x| x.id)
                        .collect::<Vec<i32>>(),
                    data.get(|x| x.city)
                        .into_iter()
                        .map(|x| x.id)
                        .collect::<Vec<i32>>(),
                )
            })
            .collect::<Vec<(i32, Vec<i32>, Vec<i32>)>>();

        let expected = vec![
            (1, vec![1], vec![2]),
            (2, vec![2], vec![3]),
            (3, vec![3, 4], vec![3]),
        ];

        assert_eq!(expected, output);
    })
}

#[test]
fn should_return_borrowed_objects_from_iterator() {
    async_std::task::block_on(async {
        let conn = get_conn().await;

        let dataset = Team::all().include(|x| x.players).run(&conn).await.unwrap();

        let output = dataset
            .iter()
            .map(|data| (data.as_ref(), data.get(|x| x.players)))
            .collect::<Vec<(&Team, Vec<&Player>)>>();

        assert_eq!(output[0].0.id, 1)
    })
}

#[test]
fn should_return_owned_objects_from_iterator() {
    async_std::task::block_on(async {
        let conn = get_conn().await;

        let dataset = Team::all().include(|x| x.players).run(&conn).await.unwrap();

        let output = dataset
            .iter()
            .map(|data| (data.clone(), data.get_owned(|x| x.players)))
            .collect::<Vec<(Team, Vec<Player>)>>();

        assert_eq!(output[0].0.id, 1)
    })
}
