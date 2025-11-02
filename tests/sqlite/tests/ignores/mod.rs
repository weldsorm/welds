use super::get_conn;
use sqlite_test::models::order::{Order, SmallOrder};
use sqlite_test::models::product::{BadProduct1, BadProduct2, Product};
use sqlite_test::models::StringThing;
use sqlite_test::models::{Thing1, Thing2, Thing3};
use welds::connections::sqlite::SqliteClient;
use welds::connections::TransactStart;
use welds::state::{DbState, DbStatus};
use welds::Syntax;

#[test]
fn should_be_able_to_select_a_readonly_field() {
    async_std::task::block_on(async {
        use sqlite_test::models::product::ProductNameOnly;
        let conn = get_conn().await;
        let product = ProductNameOnly::where_col(|p| p.description.not_equal(None))
            .fetch_one(&conn)
            .await
            .unwrap();
        assert!(product.description.is_some());
    })
}

#[test]
fn should_not_update_changes_to_readonly_field() {
    async_std::task::block_on(async {
        use sqlite_test::models::product::ProductNameOnly;
        let conn = get_conn().await;
        let mut product = ProductNameOnly::where_col(|p| p.description.not_equal(None))
            .fetch_one(&conn)
            .await
            .unwrap();
        product.description = None;
        product.save(&conn).await.unwrap();
        let product = ProductNameOnly::find_by_id(&conn, product.id)
            .await
            .unwrap()
            .unwrap();
        assert!(product.description.is_some());
    })
}

#[test]
fn should_not_insert_to_readonly_field() {
    async_std::task::block_on(async {
        use sqlite_test::models::product::ProductNameOnly;
        let conn = get_conn().await;

        let mut product = ProductNameOnly::new();
        product.name = "Test".to_string();
        product.description = Some("Test".to_string());
        product.save(&conn).await.unwrap();
        //re-pull the model from the database
        let product = ProductNameOnly::find_by_id(&conn, product.id)
            .await
            .unwrap()
            .unwrap();
        // description should not be include in the insert
        assert!(product.description.is_none());
    })
}

#[test]
fn should_not_insert_ignore_insert() {
    async_std::task::block_on(async {
        use sqlite_test::models::product::ProductInsertIgnoreDesc as Product;
        let conn = get_conn().await;

        let mut product = Product::new();
        product.description = "VALUE NOT SAVED".to_owned();
        product.save(&conn).await.unwrap();

        // verify value is not inserted
        product = Product::find_by_id(&conn, product.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(product.description, "");

        // verify value is updated
        product.description = "VALUE IS UPDATED".to_owned();
        product.save(&conn).await.unwrap();
        product = Product::find_by_id(&conn, product.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(product.description, "VALUE IS UPDATED");
    })
}

#[test]
fn should_not_update_ignore_update() {
    async_std::task::block_on(async {
        use sqlite_test::models::product::ProductUpdateIgnoreDesc as Product;
        let conn = get_conn().await;

        let mut product = Product::new();
        product.description = "INSERT".to_owned();
        product.save(&conn).await.unwrap();

        // verify value is not inserted
        product = Product::find_by_id(&conn, product.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(product.description, "INSERT");

        // verify value is updated
        product.description = "UPDATED".to_owned();
        product.save(&conn).await.unwrap();
        product = Product::find_by_id(&conn, product.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(product.description, "INSERT");
    })
}

#[test]
fn should_fully_ignore_field() {
    async_std::task::block_on(async {
        use sqlite_test::models::product::Product;
        use sqlite_test::models::product::ProductFullIgnoreDesc as ProductIgnore;
        let conn = get_conn().await;

        let mut product = ProductIgnore::new();
        product.description = "INSERT".to_owned();
        product.save(&conn).await.unwrap();

        // verify value is not inserted
        let real_product = Product::find_by_id(&conn, product.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(real_product.description, None);

        // verify value is not updated
        product.description = "UPDATED".to_owned();
        product.save(&conn).await.unwrap();
        let real_product = Product::find_by_id(&conn, product.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(real_product.description, None);

        // write a value to the description field
        Product::where_col(|p| p.id.equal(product.id))
            .set(|p| p.description, "bla")
            .run(&conn)
            .await
            .unwrap();
        // verify value is not selected
        let product = ProductIgnore::find_by_id(&conn, product.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(product.description, "");
    })
}
