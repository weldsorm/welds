pub mod mssql;
pub mod mysql;
pub mod postgres;

//pub fn wait_for_ready() {
//    // make sure everything is booting before waiting
//    let _ = postgres::init();
//    // block until all report ready
//    postgres::wait_with_ready();
//}
