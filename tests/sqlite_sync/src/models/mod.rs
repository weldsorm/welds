pub mod order;
pub mod product;
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
#[welds(table = "Thing3")]
pub struct Thing3 {
    #[welds(primary_key)]
    // we should still be able to use the table with a small type
    pub id: i16,
    pub value: String,
}

#[derive(Debug, WeldsModel)]
#[welds(table = "stringthing")]
pub struct StringThing {
    #[welds(primary_key)]
    pub id: String,
    pub value: String,
}

#[derive(Debug, WeldsModel, PartialEq, Eq)]
#[welds(table = "Users")]
#[welds(HasOne(profile, Profile, "user_id"))]
pub struct User {
    #[welds(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Debug, WeldsModel, PartialEq, Eq)]
#[welds(table = "Profiles")]
#[welds(BelongsTo(user, User, "user_id"))]
pub struct Profile {
    #[welds(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub image_url: String,
}
