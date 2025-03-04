use std::borrow::Borrow;
use std::collections::HashMap;

use crate::model_traits::{HasSchema, PkValue, TableColumns};
use crate::relations::{BelongsTo, BelongsToFkValue, HasMany, Related};

/// Combines two sets of related objects into a Vec of structs, by given lambda. Eg:
/// ```rust,ignore
/// #[derive(WeldsModel)]
/// #[welds(table = "teams")]
/// #[welds(HasMany(players, Player, "id"))]
/// struct Team {
///     #[welds(primary_key)]
///     id: i64,
///     name: String
/// }
///
/// #[derive(WeldsModel)]
/// #[welds(table = "players")]
/// #[welds(BelongsTo(team, Team, "team_id"))]
/// struct Player {
///     #[welds(primary_key)]
///     id: i64,
///     team_id: i64,
///     name: String
/// }
///
/// struct TeamWithPlayers {
///     team: Team,
///     players: Vec<Player>,
/// }
///
/// fn teams_with_players(&dyn Client) -> Result<Vec<TeamWithPlayers>> {
///     let all_teams = Team::all().run(&client).await?.into_inners();
///     let all_player = Player::all().run(&client).await?.into_inners();
///
///     let lambda = |team, players| { TeamWithPlayers { team, players } };
///     let combined = combine_related(lambda, &all_teams, &all_players);
///
///     Ok(combined)
/// }
/// ```
pub fn combine_related<OutputStruct, Parent, Child>(
    lambda: impl Fn(Parent, Vec<Child>) -> OutputStruct,
    parents: &[Parent],
    children: &[Child],
) -> Vec<OutputStruct>
where
    Parent: HasSchema + Related<HasMany<Child>> + PkValue + ToOwned<Owned = Parent>,
    <Parent as HasSchema>::Schema: TableColumns,
    Child: HasSchema + Related<BelongsTo<Parent>> + BelongsToFkValue<Parent> + ToOwned<Owned = Child>,
    <Child as HasSchema>::Schema: TableColumns,
    <Parent as PkValue>::PkVal: Borrow<<Child as BelongsToFkValue<Parent>>::FkVal>
{
    group_related(parents, children)
        .into_iter()
        .zip(parents)
        .map(|(children, parent)| lambda(parent.to_owned(), children))
        .collect()
}

/// Groups a set of objects with a set of related objects for use with `.zip()`
pub fn group_related<Parent, Child>(parents: &[Parent], children: &[Child]) -> Vec<Vec<Child>>
where
    Parent: HasSchema + Related<HasMany<Child>> + PkValue,
    <Parent as HasSchema>::Schema: TableColumns,
    Child: HasSchema + Related<BelongsTo<Parent>> + BelongsToFkValue<Parent> + ToOwned<Owned = Child>,
    <Child as HasSchema>::Schema: TableColumns,
    <Parent as PkValue>::PkVal: Borrow<<Child as BelongsToFkValue<Parent>>::FkVal>
{
    let mut grouped = Vec::new();

    let indexed: HashMap<_, _> = parents
        .iter()
        .enumerate()
        .map(|(index, parent)| {
            grouped.push(Vec::new());
            (parent.pk_value(), index)
        }).collect();

    for child in children {
        if let Some(index) = indexed.get(&child.fk_value::<Parent>()) {
            grouped[*index].push(child.to_owned());
        }
    }

    grouped
}
