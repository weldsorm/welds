use std::process::Command;

pub fn main() {
    let container_cmd = container_cmd().expect("Docker or Podman is required");

    // Make sure the Postgres Image is built
    let mut docker = Command::new(container_cmd);
    let outs = docker
        .arg("build")
        .arg("./databases/postgres")
        .arg("-t")
        .arg("welds_pg_testing_db")
        .output()
        .expect("failed to build PG test image");
    if !outs.status.success() {
        eprintln!("DOCKER BUILD ERROR: {:?}", outs);
        panic!("Docker Build Failed");
    }

    // Make sure the Mssql Image is built
    let mut docker = Command::new(container_cmd);
    let outs = docker
        .arg("build")
        .arg("./databases/mssql")
        .arg("-t")
        .arg("welds_mssql_testing_db")
        .output()
        .expect("failed to build Mssql test image");
    if !outs.status.success() {
        eprintln!("DOCKER BUILD ERROR: {:?}", outs);
        panic!("Docker Build Failed");
    }

    // Make sure the Mysql Image is built
    let mut docker = Command::new(container_cmd);
    let outs = docker
        .arg("build")
        .arg("./databases/mysql")
        .arg("-t")
        .arg("welds_mysql_testing_db")
        .output()
        .expect("failed to build MySql test image");
    if !outs.status.success() {
        eprintln!("DOCKER BUILD ERROR: {:?}", outs);
        panic!("Docker Build Failed");
    }
}

fn container_cmd() -> Option<&'static str> {
    if has_podman() {
        return Some("podman");
    } else if has_docker() {
        return Some("docker");
    }
    None
}

fn has_podman() -> bool {
    let output = Command::new("podman").arg("--version").output();
    output.is_ok()
}

fn has_docker() -> bool {
    let output = Command::new("docker").arg("--version").output();
    output.is_ok()
}
