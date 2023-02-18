pub mod database;
pub mod errors;
pub mod query;
pub mod table;

#[test]
fn failing_test() {
    let conn = testlib::postgres::conn();
    assert!(false)
}

#[test]
fn passing_test() {
    let conn = testlib::postgres::conn();
    assert!(true)
}

#[test]
fn panic_test() {
    let conn = testlib::postgres::conn();
    panic!();
    assert!(true)
}
