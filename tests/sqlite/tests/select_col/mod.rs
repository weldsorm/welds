type Db = sqlx::Sqlite;
use welds::Syntax;
use welds::WeldsModel;

#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Sqlite))]
#[welds(table = "orders")]
#[welds(BelongsTo(product, Product2, "product_id"))]
pub struct Order2 {
    #[welds(primary_key)]
    pub oid: i32,
    pub product_id: i32,
    #[sqlx(rename = "name2")]
    pub name: String,
}
#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Sqlite))]
#[welds(table = "Products")]
#[welds(HasMany(orders, Order2, "product_id"))]
pub struct Product2 {
    #[welds(primary_key)]
    pub pid: i32,
    pub name: String,
}

#[test]
fn should_be_able_to_select_a_single_column() {
    async_std::task::block_on(async {
        assert!(false);
        //let q = Order2::all::<Db>().select(|o| o.oid).to_sql();
        //assert_eq!(q, "SELECT t1.\"oid\" FROM orders t1");
    })
}

#[test]
fn should_select_with_as_to_make_returned_value_match_fieldname() {
    async_std::task::block_on(async {
        assert!(false);
        //let q = Order2::all::<Db>().select(|o| o.name).to_sql();
        //assert_eq!(q, "SELECT t1.\"name2\" as \"name\" FROM orders t1");
    })
}

#[test]
fn should_be_able_to_select_multiple_columns() {
    async_std::task::block_on(async {
        assert!(false);
        //let q = Order2::all::<Db>()
        //    .select(|o| o.oid)
        //    .select(|o| o.name)
        //    .to_sql();
        //assert_eq!(
        //    q,
        //    "SELECT t1.\"oid\", t1.\"name2\" as \"name\" FROM orders t1"
        //);
    })
}

#[test]
fn should_be_able_to_select_from_joined_table_belongs_to() {
    async_std::task::block_on(async {
        assert!(false);
        //let q = Order2::all::<Db>()
        //    .select(|o| o.oid)
        //    .join(|o| o.product, Product2::select(|p| p.pid))
        //    .to_sql();
        //assert_eq!(
        //    q,
        //    "SELECT t1.\"oid\", t2.\"pid\" FROM orders t1 JOIN Products t2 ON t1.\"product_id\" = t2.\"pid\""
        //);
    })
}

#[test]
fn should_be_able_to_select_from_joined_table_has_many() {
    async_std::task::block_on(async {
        assert!(false);
        //let q = Product2::all::<Db>()
        //    .select(|o| o.pid)
        //    .join(|o| o.orders, Order2::select(|p| p.oid))
        //    .to_sql();
        //assert_eq!(
        //    q,
        //    "SELECT t1.\"pid\", t2.\"oid\" FROM Products t1 JOIN orders t2 ON t1.\"pid\" = t2.\"product_id\""
        //);
    })
}

#[test]
fn should_be_able_to_select_join_with_where() {
    async_std::task::block_on(async {
        assert!(false);
        //let q = Order2::all::<Db>()
        //    .select(|o| o.oid)
        //    .where_col(|x| x.oid.equal(1))
        //    .join(|o| o.product, Product2::select(|p| p.pid))
        //    .to_sql();
        //assert_eq!(
        //    q,
        //    "SELECT t1.\"oid\", t2.\"pid\" FROM orders t1 JOIN Products t2 ON t1.\"product_id\" = t2.\"pid\" WHERE ( t1.oid = ? )"
        //);
    })
}

#[test]
fn should_be_able_to_select_where_in_join() {
    async_std::task::block_on(async {
        assert!(false);
        //let q = Order2::all::<Db>()
        //    .select(|o| o.oid)
        //    .join(
        //        |o| o.product,
        //        Product2::select(|p| p.pid).where_col(|x| x.pid.equal(1)),
        //    )
        //    .to_sql();
        //assert_eq!(
        //    q,
        //    "SELECT t1.\"oid\", t2.\"pid\" FROM orders t1 JOIN Products t2 ON t1.\"product_id\" = t2.\"pid\" WHERE ( t2.pid = ? )"
        //);
    })
}
