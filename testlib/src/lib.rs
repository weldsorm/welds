pub mod postgres;

//#[test]
//fn internal_tests() {
//    let _conn = postgres::conn();
//    assert!(false, "IS_READY: {}", false);
//}

pub fn wait_for_ready() {
    // make sure everything is booting before waiting
    let _ = postgres::init();
    // block until all report ready
    postgres::wait_with_ready();
}
