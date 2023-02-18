pub mod postgres;

#[test]
fn internal_tests() {
    let _conn = postgres::conn();
    assert!(false, "IS_READY: {}", false);
}

//#[test]
//fn internal_tests() {
//    let is_ready = {
//        let pg = Postgres::new().unwrap();
//        pg.wait_for_ready().unwrap();
//        pg.is_ready()
//    };
//    assert!(false, "IS_READY: {}", is_ready);
//}
