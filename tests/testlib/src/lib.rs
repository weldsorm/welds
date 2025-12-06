pub mod mssql;
pub mod mysql;
pub mod postgres;

pub(crate) fn container_cmd() -> Option<&'static str> {
    if has_podman() {
        return Some("podman");
    } else if has_docker() {
        return Some("docker");
    }
    None
}

use std::process::Command;

fn has_podman() -> bool {
    let output = Command::new("podman").arg("--version").output();
    output.is_ok()
}

fn has_docker() -> bool {
    let output = Command::new("docker").arg("--version").output();
    output.is_ok()
}
