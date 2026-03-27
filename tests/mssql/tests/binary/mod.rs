use super::get_conn;
use mssql_test::models::binary::{BinaryA, BinaryKey};
use uuid::Uuid;
use welds::prelude::*;

#[tokio::test]
async fn should_be_able_to_curd_binary_table() {
    let conn = get_conn().await;
    let id = Uuid::new_v4();
    let id2 = Uuid::new_v4();
    let bytes = id.to_bytes_le().to_vec();
    let bytes2 = id2.to_bytes_le().to_vec();

    let mut obj = DbState::new_uncreated(BinaryA {
        id,
        b1: bytes.clone(),
        b2: bytes.clone(),
        b3: bytes.clone(),
        ob1: None,
        ob2: None,
        ob3: None,
    });
    let obj_original: BinaryA = obj.clone();

    // Create / Load
    obj.save(&conn).await.unwrap();
    let loaded = BinaryA::find_by_id(&conn, id).await.unwrap().unwrap();
    assert_eq!(obj, loaded);
    assert_eq!(&obj_original, loaded.as_ref());
    // Update / Load
    obj.b1 = bytes2.clone();
    obj.b2 = bytes2.clone();
    obj.b3 = bytes2.clone();
    obj.save(&conn).await.unwrap();
    let loaded = BinaryA::find_by_id(&conn, id).await.unwrap().unwrap();
    assert_eq!(obj, loaded);
    // Delete / non-Load
    obj.delete(&conn).await.unwrap();
    let loaded = BinaryA::find_by_id(&conn, id).await.unwrap();
    assert!(loaded.is_none());
}

#[tokio::test]
async fn should_be_able_to_read_write_binary_pks() {
    let conn = get_conn().await;
    let id = Uuid::new_v4();
    let bytes = id.to_bytes_le().to_vec();
    let mut obj = DbState::new_uncreated(BinaryKey { id: bytes.clone() });

    // Create / Load
    obj.save(&conn).await.unwrap();
    let loaded = BinaryKey::find_by_id(&conn, bytes).await.unwrap().unwrap();
    assert_eq!(obj, loaded);
}
