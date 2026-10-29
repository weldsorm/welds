use super::get_conn;
use sqlite_test::models::order::Order;
use sqlite_test::models::product::Product;
use welds::Syntax;
use welds::WeldsModel;

#[derive(Debug, WeldsModel)]
#[welds(db(Sqlite))]
#[welds(table = "orders")]
#[welds(BelongsTo(product, Product2, "product_id"))]
pub struct Order2 {
    #[welds(primary_key)]
    pub oid: i32,
    pub product_id: i32,
    #[welds(rename = "name2")]
    pub name: String,
}
#[derive(Debug, WeldsModel)]
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
        let q = Order2::all().select(|o| o.oid).to_sql(Syntax::Sqlite);
        assert_eq!(q, "SELECT t1.\"oid\" FROM orders t1");
    })
}

#[test]
fn should_select_with_as_to_make_returned_value_match_fieldname() {
    async_std::task::block_on(async {
        let q = Order2::all().select(|o| o.name).to_sql(Syntax::Sqlite);
        assert_eq!(q, "SELECT t1.\"name2\" AS \"name\" FROM orders t1");
    })
}

#[test]
fn should_be_able_to_select_multiple_columns() {
    async_std::task::block_on(async {
        let q = Order2::all()
            .select(|o| o.oid)
            .select(|o| o.name)
            .to_sql(Syntax::Sqlite);
        assert_eq!(
            q,
            "SELECT t1.\"oid\", t1.\"name2\" AS \"name\" FROM orders t1"
        );
    })
}

#[test]
fn should_be_able_to_select_from_joined_table_belongs_to() {
    async_std::task::block_on(async {
        let q = Order2::all()
            .select(|o| o.oid)
            .join(|o| o.product, Product2::select(|p| p.pid))
            .to_sql(Syntax::Sqlite);
        assert_eq!(
            q,
            "SELECT t1.\"oid\", t2.\"pid\" FROM orders t1 JOIN Products t2 ON t1.\"product_id\" = t2.\"pid\""
        );
    })
}

#[test]
fn should_be_able_to_select_from_joined_table_has_many() {
    async_std::task::block_on(async {
        let q = Product2::all()
            .select(|o| o.pid)
            .join(|o| o.orders, Order2::select(|p| p.oid))
            .to_sql(Syntax::Sqlite);
        assert_eq!(
            q,
            "SELECT t1.\"pid\", t2.\"oid\" FROM Products t1 JOIN orders t2 ON t1.\"pid\" = t2.\"product_id\""
        );
    })
}

#[test]
fn should_be_able_to_select_join_with_where() {
    async_std::task::block_on(async {
        let q = Order2::all()
            .select(|o| o.oid)
            .where_col(|x| x.oid.equal(1))
            .join(|o| o.product, Product2::select(|p| p.pid))
            .to_sql(Syntax::Sqlite);
        assert_eq!(
            q,
            "SELECT t1.\"oid\", t2.\"pid\" FROM orders t1 JOIN Products t2 ON t1.\"product_id\" = t2.\"pid\" WHERE ( t1.\"oid\" = ? )"
        );
    })
}

#[test]
fn should_be_able_to_select_where_in_join() {
    async_std::task::block_on(async {
        let q = Order2::all()
            .select(|o| o.oid)
            .join(
                |o| o.product,
                Product2::select(|p| p.pid).where_col(|x| x.pid.equal(1)),
            )
            .to_sql(Syntax::Sqlite);
        assert_eq!(
            q,
            "SELECT t1.\"oid\", t2.\"pid\" FROM orders t1 JOIN Products t2 ON t1.\"product_id\" = t2.\"pid\" WHERE ( t2.\"pid\" = ? )"
        );
    })
}

#[test]
fn should_be_able_to_select_out_columns_of_the_name_name() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let rows: Result<Vec<_>, _> = Product::select_as(|p| p.id, "product_id")
            .join(|p| p.orders, Order::select_as(|o| o.id, "order_id"))
            .run(&conn)
            .await
            .unwrap()
            .drain(..)
            .map(|r| (r.get::<i32>("product_id")))
            .collect();
        let _rows = rows.unwrap();
    })
}

#[test]
fn should_be_able_to_select_all_columns() {
    async_std::task::block_on(async {
        let query = Order2::all().select_all();

        assert_eq!(query.to_sql(Syntax::Sqlite), "SELECT t1.* FROM orders t1");
    });
}

#[test]
fn should_be_able_to_select_column_count() {
    async_std::task::block_on(async {
        let query = Order2::all().select_count(|o| o.oid, "oid_count");

        assert_eq!(
            query.to_sql(Syntax::Sqlite),
            "SELECT COUNT(t1.\"oid\") AS \"oid_count\" FROM orders t1"
        );
    });
}

#[test]
fn should_be_able_to_select_column_max() {
    async_std::task::block_on(async {
        let query = Order2::all().select_max(|o| o.oid, "oid_max");

        assert_eq!(
            query.to_sql(Syntax::Sqlite),
            "SELECT MAX(t1.\"oid\") AS \"oid_max\" FROM orders t1"
        );
    });
}

#[test]
fn should_be_able_to_select_column_min() {
    async_std::task::block_on(async {
        let query = Order2::all().select_min(|o| o.oid, "oid_min");

        assert_eq!(
            query.to_sql(Syntax::Sqlite),
            "SELECT MIN(t1.\"oid\") AS \"oid_min\" FROM orders t1"
        );
    });
}

#[test]
fn should_be_able_to_select_column_avg() {
    async_std::task::block_on(async {
        let query = Order2::all().select_avg(|o| o.oid, "oid_avg");

        assert_eq!(
            query.to_sql(Syntax::Sqlite),
            "SELECT AVG(t1.\"oid\") AS \"oid_avg\" FROM orders t1"
        );
    });
}

#[test]
fn should_be_able_to_select_column_sum() {
    async_std::task::block_on(async {
        let query = Order2::all().select_sum(|o| o.oid, "oid_sum");

        assert_eq!(
            query.to_sql(Syntax::Sqlite),
            "SELECT SUM(t1.\"oid\") AS \"oid_sum\" FROM orders t1"
        );
    });
}

#[test]
fn should_be_able_to_join_and_group_by() {
    async_std::task::block_on(async {
        let query = Order2::all()
            .select(|o| o.oid)
            .left_join(
                |o| o.product,
                Product2::all().select_count(|p| p.pid, "product_count"),
            )
            .group_by(|o| o.oid);

        assert_eq!(
            query.to_sql(Syntax::Sqlite),
            "SELECT t1.\"oid\", COUNT(t2.\"pid\") AS \"product_count\" \
            FROM orders t1 \
            LEFT JOIN Products t2 ON t1.\"product_id\" = t2.\"pid\" \
            GROUP BY t1.\"oid\""
        );
    });
}

#[test]
fn should_be_able_to_group_by_multiple_columns_from_either_side_of_a_join() {
    async_std::task::block_on(async {
        let query = Order2::all()
            .select(|o| o.oid)
            .left_join(|o| o.product, {
                Product2::all()
                    .select_count(|p| p.pid, "product_count")
                    .group_by(|p| p.name)
            })
            .group_by(|o| o.oid);

        let sql_a = "SELECT t1.\"oid\", COUNT(t2.\"pid\") AS \"product_count\" FROM orders t1 LEFT JOIN Products t2 ON t1.\"product_id\" = t2.\"pid\" GROUP BY t1.\"oid\", t2.\"name\"";
        let sql_b = "SELECT t1.\"oid\", COUNT(t2.\"pid\") AS \"product_count\" FROM orders t1 LEFT JOIN Products t2 ON t1.\"product_id\" = t2.\"pid\" GROUP BY t2.\"name\", t1.\"oid\"";
        let sql = query.to_sql(Syntax::Sqlite);
        //both are valid
        assert!(sql == sql_a || sql == sql_b);
    });
}

#[test]
fn should_be_able_to_write_order_by_with_table_and_column_aliases() {
    async_std::task::block_on(async {
        let query = Order2::all()
            .select(|o| o.oid)
            .select_as(|o| o.name, "aliased_column")
            .left_join(|o| o.product, { Product2::all().select(|p| p.pid) })
            .order_manual("$.oid DESC")
            .order_manual("t2.pid DESC")
            .order_manual("aliased_column ASC");

        assert_eq!(
            query.to_sql(Syntax::Sqlite),
            "SELECT t1.\"oid\", t1.\"name2\" AS \"aliased_column\", t2.\"pid\" \
            FROM orders t1 \
            LEFT JOIN Products t2 ON t1.\"product_id\" = t2.\"pid\" \
            ORDER BY t1.oid DESC, t2.pid DESC, aliased_column ASC"
        );
    });
}

#[derive(Debug, WeldsModel)]
#[welds(db(Sqlite))]
#[welds(table = "Products")]
#[welds(HasMany(orders, Order3, "product_id"))]
pub struct Product3 {
    #[welds(primary_key)]
    #[welds(rename = "product_id")]
    pub id: i32,
    pub name: String,
}

#[derive(Debug, WeldsModel)]
#[welds(db(Sqlite))]
#[welds(table = "Orders")]
#[welds(BelongsTo(product, Product3, "product_id"))]
pub struct Order3 {
    #[welds(primary_key)]
    pub id: i32,
    pub product_id: i32,
    pub product_id2: Option<i32>,
    pub price: Option<f32>,
}

#[derive(Debug, Default, WeldsModel)]
#[welds(db(Sqlite))]
pub struct ProductOrderRow {
    pub pid: i32,
    pub oid: Option<i32>,
}

#[test]
fn should_be_able_to_left_join_on_multiple_keys() {
    async_std::task::block_on(async {
        let conn = get_conn().await;

        // baseline: horse (product 1) has orders 1 and 2
        let rows: Vec<ProductOrderRow> = Product3::where_col(|p| p.id.equal(1))
            .select_as(|p| p.id, "pid")
            .left_join(|p| p.orders, Order3::all().select_as(|o| o.id, "oid"))
            .run(&conn)
            .await
            .unwrap()
            .into_iter()
            .map(|r| r.try_into().unwrap())
            .collect();
        assert_eq!(rows.len(), 2);

        // second key on the ON clause: no order has product_id2 set,
        // so the LEFT JOIN keeps the product with a NULL order
        let q = Product3::where_col(|p| p.id.equal(1))
            .select_as(|p| p.id, "pid")
            .left_join_on(
                |p| p.orders,
                Order3::all().select_as(|o| o.id, "oid"),
                |on| on.eq(|p| p.id, |o| o.product_id2),
            );
        assert_eq!(
            q.to_sql(Syntax::Sqlite),
            "SELECT t1.\"product_id\" AS \"pid\", t2.\"id\" AS \"oid\" FROM Products t1 \
            LEFT JOIN Orders t2 ON t1.\"product_id\" = t2.\"product_id\" AND t1.\"product_id\" = t2.\"product_id2\" \
            WHERE ( t1.\"product_id\" = ? )"
        );
        let rows: Vec<ProductOrderRow> = q
            .run(&conn)
            .await
            .unwrap()
            .into_iter()
            .map(|r| r.try_into().unwrap())
            .collect();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].pid, 1);
        assert_eq!(rows[0].oid, None);
    })
}

#[test]
fn on_clause_params_bind_before_where_params() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        // ON param (price > 20) must bind before WHERE param (name = 'horse')
        let rows: Vec<ProductOrderRow> = Product3::where_col(|p| p.name.equal("horse"))
            .select_as(|p| p.id, "pid")
            .left_join_on(
                |p| p.orders,
                Order3::all().select_as(|o| o.id, "oid"),
                |on| on.inner(|o| o.price.gt(20.0)),
            )
            .run(&conn)
            .await
            .unwrap()
            .into_iter()
            .map(|r| r.try_into().unwrap())
            .collect();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].oid, Some(2));
    })
}
