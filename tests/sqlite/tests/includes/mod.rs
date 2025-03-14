use crate::get_conn;
use welds::Syntax;
use welds::WeldsModel;
use std::ops::Deref;

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

#[test]
fn should_load_included_with_has_one() {
    async_std::task::block_on(async {
        let conn = get_conn().await;

        let dataset = User::all().include(|x| x.profile).run(&conn).await.unwrap();

        let output = dataset.iter().map(|data| {
            (
                data.deref().id,
                data.get(|x| x.profile).into_iter().map(|x| x.id).collect::<Vec<i32>>()
            )
        }).collect::<Vec<(i32, Vec<i32>)>>();

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

        let output = dataset.iter().map(|data| {
            (
                data.deref().id,
                data.get(|x| x.user).into_iter().map(|x| x.id).collect::<Vec<i32>>()
            )
        }).collect::<Vec<(i32, Vec<i32>)>>();

        let expected = vec![
            (1, vec![1]),
            (2, vec![3]),
            (3, vec![4]),
        ];

        assert_eq!(expected, output);
    })
}
