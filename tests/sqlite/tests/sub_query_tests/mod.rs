use super::get_conn;
use welds::connections::TransactStart;
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
fn exist_in_mapping_query_source_belongs() {
    async_std::task::block_on(async {
        let q1 = Order2::all()
            .map_query(|o| o.product)
            .to_sql(Syntax::Sqlite);
        assert_eq!(q1, "SELECT t2.\"pid\", t2.\"name\" FROM Products t2 WHERE ( EXISTS ( SELECT product_id FROM orders t1 WHERE t1.product_id = t2.pid ) )");
        let q2 = Order2::all()
            .limit(1)
            .map_query(|o| o.product)
            .to_sql(Syntax::Sqlite);
        assert_eq!(q2, "SELECT t2.\"pid\", t2.\"name\" FROM Products t2 WHERE (  t2.pid IN (SELECT t1.product_id FROM orders t1  ORDER BY 1 LIMIT 1 OFFSET 0 )  )");
    })
}

#[test]
fn should_be_able_to_join_limit_and_order_all_at_once() {
    async_std::task::block_on(async {
        let sql = Order2::all()
            .limit(1)
            .order_by_asc(|o| o.oid)
            .map_query(|o| o.product)
            .to_sql(Syntax::Sqlite);
        let expected = "SELECT t2.\"pid\", t2.\"name\" FROM Products t2 WHERE (  t2.pid IN (SELECT t1.product_id FROM orders t1  ORDER BY t1.oid ASC LIMIT 1 OFFSET 0 )  )";
        assert_eq!(expected, sql);
    })
}

#[test]
fn exist_in_mapping_query_source_many() {
    async_std::task::block_on(async {
        let q1 = Product2::all()
            .map_query(|o| o.orders)
            .to_sql(Syntax::Sqlite);
        assert_eq!(q1, "SELECT t2.\"oid\", t2.\"product_id\" FROM orders t2 WHERE ( EXISTS ( SELECT pid FROM Products t1 WHERE t1.pid = t2.product_id ) )");
        let q2 = Product2::all()
            .limit(1)
            .map_query(|o| o.orders)
            .to_sql(Syntax::Sqlite);
        assert_eq!(q2, "SELECT t2.\"oid\", t2.\"product_id\" FROM orders t2 WHERE (  t2.product_id IN (SELECT t1.pid FROM Products t1  ORDER BY 1 LIMIT 1 OFFSET 0 )  )");
    })
}

#[test]
fn exist_in_sub_query_source_belongs() {
    async_std::task::block_on(async {
        let sub = Order2::all();
        let q1 = Product2::all()
            .where_relation(|o| o.orders, sub)
            .to_sql(Syntax::Sqlite);
        assert_eq!(q1, "SELECT t1.\"pid\", t1.\"name\" FROM Products t1 WHERE ( EXISTS ( SELECT product_id FROM orders t2 WHERE t2.product_id = t1.pid ) )");

        let sub2 = Order2::all().limit(1);
        let q2 = Product2::all()
            .where_relation(|o| o.orders, sub2)
            .to_sql(Syntax::Sqlite);
        assert_eq!(q2, "SELECT t1.\"pid\", t1.\"name\" FROM Products t1 WHERE (  t1.pid IN (SELECT t2.product_id FROM orders t2  ORDER BY 1 LIMIT 1 OFFSET 0 )  )");
    })
}

#[test]
fn exist_in_sub_query_source_many() {
    async_std::task::block_on(async {
        let sub1 = Product2::all();
        let q1 = Order2::all()
            .where_relation(|o| o.product, sub1)
            .to_sql(Syntax::Sqlite);
        assert_eq!(q1, "SELECT t1.\"oid\", t1.\"product_id\" FROM orders t1 WHERE ( EXISTS ( SELECT pid FROM Products t2 WHERE t2.pid = t1.product_id ) )");

        let sub2 = Product2::all().limit(1);
        let q2 = Order2::all()
            .where_relation(|o| o.product, sub2)
            .to_sql(Syntax::Sqlite);
        assert_eq!(q2, "SELECT t1.\"oid\", t1.\"product_id\" FROM orders t1 WHERE (  t1.product_id IN (SELECT t2.pid FROM Products t2  ORDER BY 1 LIMIT 1 OFFSET 0 )  )");
    })
}

#[test]
fn three_levels_down() {
    async_std::task::block_on(async {
        let q = Product2::all()
            .where_col(|x| x.pid.equal(1))
            .map_query(|p| p.orders)
            .where_col(|o| o.oid.equal(2))
            .map_query(|o| o.product)
            .where_col(|p| p.pid.equal(3))
            .to_sql(Syntax::Sqlite);

        assert_eq!(q, "SELECT t3.\"pid\", t3.\"name\" FROM Products t3 WHERE ( t3.pid = ? AND EXISTS ( SELECT product_id FROM orders t2 WHERE t2.oid = ? AND t2.product_id = t3.pid AND EXISTS ( SELECT pid FROM Products t1 WHERE t1.pid = ? AND t1.pid = t2.product_id ) ) )");
    })
}

#[test]
fn should_be_able_to_query_one_to_one_with_where_relation_from_source() {
    use sqlite_test::models::Profile;
    use sqlite_test::models::User;

    async_std::task::block_on(async {
        let conn = get_conn().await;

        let sub_query_profile = Profile::where_col(|p| p.image_url.equal("bird.png"));
        let query = User::all().where_relation(|u| u.profile, sub_query_profile);
        eprintln!("QUERY: {}", query.to_sql(Syntax::Sqlite));
        let data = query.run(&conn).await.unwrap();

        assert_eq!(data.len(), 1);
        assert_eq!(data[0].id, 4);
    })
}

#[test]
fn should_be_able_to_query_one_to_one_with_where_relation_from_desc() {
    use sqlite_test::models::Profile;
    use sqlite_test::models::User;

    async_std::task::block_on(async {
        let conn = get_conn().await;

        let sub_query_user = User::where_col(|u| u.name.equal("Danny"));
        let query = Profile::all().where_relation(|u| u.user, sub_query_user);
        eprintln!("QUERY: {}", query.to_sql(Syntax::Sqlite));
        let data = query.run(&conn).await.unwrap();

        assert_eq!(data.len(), 1);
        assert_eq!(data[0].id, 3);
    })
}

#[test]
fn should_be_able_to_query_one_to_one_with_map_query_from_source() {
    use sqlite_test::models::Profile;
    use sqlite_test::models::User;

    async_std::task::block_on(async {
        let conn = get_conn().await;

        let query = User::all()
            .map_query(|u| u.profile)
            .where_col(|p| p.image_url.equal("dog.jpeg"));
        eprintln!("QUERY: {}", query.to_sql(Syntax::Sqlite));
        let data = query.run(&conn).await.unwrap();

        assert_eq!(data.len(), 1);
        assert_eq!(data[0].id, 2);
    })
}

#[test]
fn should_be_able_to_query_one_to_one_with_map_query_from_desc() {
    use sqlite_test::models::Profile;
    use sqlite_test::models::User;

    async_std::task::block_on(async {
        let conn = get_conn().await;

        let query = Profile::all()
            .map_query(|p| p.user)
            .where_col(|u| u.name.equal("Alice"));
        eprintln!("QUERY: {}", query.to_sql(Syntax::Sqlite));
        let data = query.run(&conn).await.unwrap();

        assert_eq!(data.len(), 1);
        assert_eq!(data[0].id, 1);
    })
}
