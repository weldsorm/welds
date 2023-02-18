use std::process::{Command, Stdio};
use std::thread::sleep;

pub struct Postgres {
    container_id: String,
}

impl Postgres {
    pub fn new() -> Result<Self, String> {
        let mut pg = Self {
            container_id: String::default(),
        };
        pg.boot()?;
        Ok(pg)
    }

    fn boot(&mut self) -> Result<(), String> {
        let output = Command::new("docker")
            .arg("run")
            .arg("--rm")
            .arg("-d")
            .arg("welds_pg_testing_db")
            .output()
            .map_err(|err| format!("{:?}", err))?;
        let id = String::from_utf8_lossy(output.stdout.as_ref());
        self.container_id = id.trim().to_string();
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        let logs = Command::new("docker")
            .arg("inspect")
            .arg(&self.container_id)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let grep = Command::new("grep")
            .arg("-q")
            .arg("Error: No such object")
            .stdin(Stdio::from(logs.stdout.unwrap()))
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let output = grep.wait_with_output().unwrap();
        !output.status.success()
    }

    pub fn is_ready(&self) -> bool {
        let logs = Command::new("docker")
            .arg("logs")
            .arg(&self.container_id)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let grep = Command::new("grep")
            .arg("-q")
            .arg("database system is ready to accept connections")
            .stdin(Stdio::from(logs.stdout.unwrap()))
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let output = grep.wait_with_output().unwrap();
        output.status.success()
    }

    pub fn wait_for_ready(&self) -> Result<(), &'static str> {
        let ten_millis = std::time::Duration::from_millis(100);
        loop {
            if !self.is_running() {
                return Err("Container No Running");
            }
            if self.is_ready() {
                return Ok(());
            }
            sleep(ten_millis);
        }
    }
}

impl Drop for Postgres {
    fn drop(&mut self) {
        let _ = Command::new("docker")
            .arg("kill")
            .arg(&self.container_id)
            .output();
    }
}
