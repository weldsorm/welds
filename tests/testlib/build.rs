use std::process::Command;

pub fn main() {
    // Make sure the Postgres Image is built
    let mut docker = Command::new("docker");
    let outs = docker
        .arg("build")
        .arg("./databases/postgres")
        .arg("-t")
        .arg("welds_pg_testing_db")
        .output()
        .expect("failed to build PG test image");
    if outs.status.success() == false {
        eprintln!("DOCKER BUILD ERROR: {:?}", outs);
        panic!("Docker Build Failed");
    }

    // Make sure the Mssql Image is built
    let mut docker = Command::new("docker");
    let outs = docker
        .arg("build")
        .arg("./databases/mssql")
        .arg("-t")
        .arg("welds_mssql_testing_db")
        .output()
        .expect("failed to build Mssql test image");
    if outs.status.success() == false {
        eprintln!("DOCKER BUILD ERROR: {:?}", outs);
        panic!("Docker Build Failed");
    }

    // Make sure the Mysql Image is built
    let mut docker = Command::new("docker");
    let outs = docker
        .arg("build")
        .arg("./databases/mysql")
        .arg("-t")
        .arg("welds_mysql_testing_db")
        .output()
        .expect("failed to build MySql test image");
    if outs.status.success() == false {
        eprintln!("DOCKER BUILD ERROR: {:?}", outs);
        panic!("Docker Build Failed");
    }
}
