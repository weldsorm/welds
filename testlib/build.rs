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
}
