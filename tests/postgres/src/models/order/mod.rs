use welds_core::WeldsModel;

/*
 * NOTE: You shouldn't be writing Models by hand.
 * use the welds cli to generate models
 * The this model is here for the purpose of testing core itself
 * */

#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Postgres))]
#[welds(table = "Orders")]
pub struct Order {
    #[welds(primary_key)]
    pub id: i64,
    pub product_id: Option<i32>,
    pub quantity: Option<i16>,
    pub code: Option<String>,
    #[sqlx(rename = "SoldFor")]
    pub sold_for: Option<f64>,
}

impl welds_core::relations::HasRelations for Order {
    type Relation = OrderRelation;
}

use welds_core::relations::*;
pub struct OrderRelation {
    pub product: BelongsTo<super::product::Product>,
}

impl Default for OrderRelation {
    fn default() -> Self {
        Self {
            product: BelongsTo::using("product_id"),
        }
    }
}

impl welds_core::table::UniqueIdentifier<sqlx::Postgres> for Order {
    fn id_column() -> welds_core::table::Column {
        welds_core::table::Column::new::<sqlx::Postgres, i64>("id")
    }
}

//use welds_core::query::select::SelectBuilder;
//use welds_core::table::HasSchema;
//use welds_core::table::TableColumns;
//impl Order {
//    pub fn product<'args, DB>(&self) -> Option<SelectBuilder<'args, super::product::Product, DB>>
//    where
//        DB: sqlx::Database,
//        <super::product::Product as HasSchema>::Schema: TableColumns<DB>,
//        super::product::Product: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
//    {
//        let fk = self.product_id?;
//        let q = super::product::Product::where_col(|x| x.id.equal(fk));
//
//        q
//    }
//}
