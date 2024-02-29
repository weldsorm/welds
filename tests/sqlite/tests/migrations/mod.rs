use super::get_conn;
use welds::errors::Result;
use welds::migrations::types::Type;
use welds::migrations::{alter_table, create_table, MigrationWriter, TableState};
use welds::migrations::{down, up};
use welds::Syntax;

fn create_first_table(_state: &TableState) -> Result<Box<dyn MigrationWriter>> {
    let m = create_table("blarf")
        .id(|c| c("id", Type::Int))
        .column(|c| c("name", Type::String));
    Ok(Box::new(m))
}

#[test]
fn should_be_able_to_create_a_table() {
    async_std::task::block_on(async {
        //let client = get_conn().await;
        //let client = client.as_ref();

        //let migrations = vec![&create_first_table];
        //up(migrations).await.unwrap();
        assert!(false);
    })
}
