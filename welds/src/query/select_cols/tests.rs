use crate::Syntax;
use crate::WeldsModel;
use crate::query::clause::ParamArgs;
use crate::query::select_cols::Join;

#[derive(Debug, Default, WeldsModel)]
#[welds(table = "products")]
#[welds_path(crate)] // needed only within the welds crate.
#[welds(HasMany(orders, Order, "product_id"))]
struct Product {
    #[welds(primary_key)]
    pub id: i32,
    pub name: String,
    pub app: String,
}

#[derive(Debug, Default, WeldsModel)]
#[welds(table = "orders")]
#[welds_path(crate)] // needed only within the welds crate.
#[welds(BelongsTo(product, Product, "product_id"))]
struct Order {
    #[welds(primary_key)]
    pub id: i32,
    pub product_id: i32,
    pub price: i32,
    pub app: String,
    pub active: bool,
}

#[test]
fn should_be_able_to_select_as() {
    futures::executor::block_on(async move {
        let q = Product::all().select_as(|x| x.id, "apples");
        let sql = q.to_sql(Syntax::Postgres);
        assert_eq!(sql, "SELECT t1.\"id\" AS \"apples\" FROM products t1");
    });
}

#[test]
fn should_be_able_to_select_distinct() {
    futures::executor::block_on(async move {
        let q = Product::all().select(|x| x.name).distinct();
        let sql = q.to_sql(Syntax::Postgres);
        assert_eq!(sql, "SELECT DISTINCT t1.\"name\" FROM products t1");
    });
}

#[test]
fn should_be_able_to_select_both_sets_of_ids() {
    futures::executor::block_on(async move {
        let q = Product::all()
            .select_as(|x| x.id, "pid")
            .join(|x| x.orders, Order::all().select_as(|o| o.id, "oid"));
        let sql = q.to_sql(Syntax::Postgres);
        assert_eq!(
            sql,
            "SELECT t1.\"id\" AS \"pid\", t2.\"id\" AS \"oid\" FROM products t1 JOIN orders t2 ON t1.\"id\" = t2.\"product_id\""
        );
    });
}

#[test]
fn should_be_able_to_select_join_with_order_by() {
    futures::executor::block_on(async move {
        let q = Product::all()
            .select(|x| x.id)
            .join(|x| x.orders, Order::all().select_as(|o| o.price, "price"))
            .order_by_asc(|x| x.id);
        let sql = q.to_sql(Syntax::Postgres);
        assert_eq!(
            sql,
            "SELECT t1.\"id\", t2.\"price\" FROM products t1 JOIN orders t2 ON t1.\"id\" = t2.\"product_id\" ORDER BY t1.\"id\" ASC"
        );
    });
}

#[test]
fn should_be_able_to_left_join_on_multiple_keys() {
    let q = Product::all().select(|x| x.id).left_join_on(
        |x| x.orders,
        Order::all().select_as(|o| o.id, "oid"),
        |on| on.eq(|p| p.app, |o| o.app),
    );
    assert_eq!(
        q.to_sql(Syntax::Postgres),
        "SELECT t1.\"id\", t2.\"id\" AS \"oid\" FROM products t1 \
        LEFT JOIN orders t2 ON t1.\"id\" = t2.\"product_id\" AND t1.\"app\" = t2.\"app\""
    );
}

#[test]
fn should_be_able_to_inner_join_on_multiple_keys() {
    let q = Product::all().select(|x| x.id).join_on(
        |x| x.orders,
        Order::all().select(|o| o.price),
        |on| on.eq(|p| p.app, |o| o.app).eq(|p| p.name, |o| o.app),
    );
    assert_eq!(
        q.to_sql(Syntax::Mysql),
        "SELECT t1.id, t2.price FROM products t1 \
        JOIN orders t2 ON t1.id = t2.product_id AND t1.app = t2.app AND t1.name = t2.app"
    );
}

#[test]
fn join_with_on_respects_join_type() {
    let q = Product::all().select(|x| x.id).join_with_on(
        |x| x.orders,
        Order::all().select(|o| o.price),
        Join::Right,
        |on| on.eq(|p| p.app, |o| o.app),
    );
    assert_eq!(
        q.to_sql(Syntax::Postgres),
        "SELECT t1.\"id\", t2.\"price\" FROM products t1 \
        RIGHT JOIN orders t2 ON t1.\"id\" = t2.\"product_id\" AND t1.\"app\" = t2.\"app\""
    );
}

#[test]
fn on_params_are_numbered_before_where_params() {
    let q = Product::all()
        .select(|x| x.id)
        .where_col(|p| p.name.equal("bob"))
        .left_join_on(
            |x| x.orders,
            Order::all()
                .select(|o| o.price)
                .where_col(|o| o.price.gt(5)),
            |on| {
                on.eq(|p| p.app, |o| o.app)
                    .inner(|o| o.active.equal(true))
                    .outer(|p| p.app.equal("x"))
            },
        );
    assert_eq!(
        q.to_sql(Syntax::Postgres),
        "SELECT t1.\"id\", t2.\"price\" FROM products t1 \
        LEFT JOIN orders t2 ON t1.\"id\" = t2.\"product_id\" AND t1.\"app\" = t2.\"app\" \
        AND t2.\"active\" = $1 AND t1.\"app\" = $2 \
        WHERE ( t1.\"name\" = $3 AND t2.\"price\" > $4 )"
    );

    // the params must also be bound in that same order
    let mut args: Option<ParamArgs> = Some(Vec::default());
    let _ = q.sql_internal(Syntax::Postgres, &mut args);
    assert_eq!(args.unwrap().len(), 4);
}

#[test]
fn should_be_able_to_write_manual_on_conditions() {
    let q = Product::all().select(|x| x.id).left_join_on(
        |x| x.orders,
        Order::all().select(|o| o.price),
        |on| on.manual("{outer}.app = {inner}.app OR {inner}.app = ?", ("global",)),
    );
    assert_eq!(
        q.to_sql(Syntax::Mssql),
        "SELECT t1.\"id\", t2.\"price\" FROM products t1 \
        LEFT JOIN orders t2 ON t1.\"id\" = t2.\"product_id\" \
        AND ( t1.app = t2.app OR t2.app = @p1 )"
    );
}

#[test]
fn nested_joins_use_correct_aliases_for_on_conditions() {
    let q = Order::all().select(|o| o.id).left_join_on(
        |o| o.product,
        Product::all().select(|p| p.name).left_join_on(
            |p| p.orders,
            Order::all().select_as(|o| o.id, "other_id"),
            |on| on.eq(|p| p.app, |o| o.app),
        ),
        |on| on.eq(|o| o.app, |p| p.app),
    );
    assert_eq!(
        q.to_sql(Syntax::Postgres),
        "SELECT t1.\"id\", t2.\"name\", t3.\"id\" AS \"other_id\" FROM orders t1 \
        LEFT JOIN products t2 ON t1.\"product_id\" = t2.\"id\" AND t1.\"app\" = t2.\"app\" \
        LEFT JOIN orders t3 ON t2.\"id\" = t3.\"product_id\" AND t2.\"app\" = t3.\"app\""
    );
}

#[test]
fn plain_left_join_is_unchanged() {
    let q = Product::all()
        .select(|x| x.id)
        .left_join(|x| x.orders, Order::all().select(|o| o.price));
    assert_eq!(
        q.to_sql(Syntax::Postgres),
        "SELECT t1.\"id\", t2.\"price\" FROM products t1 LEFT JOIN orders t2 ON t1.\"id\" = t2.\"product_id\""
    );
}
