use rand::{distributions::Alphanumeric, Rng};
use static_init::dynamic;
use std::process::{Command, Stdio};
use std::thread::sleep; // 0.8

#[dynamic(drop, lazy)]
static mut MSSQL: Mssql = Mssql::new();

/// A shared connection to the testing Mssql database.
/// Automatically booted and drop as needed
pub async fn conn() -> Result<sqlx::MssqlPool, sqlx::Error> {
    let db = &MSSQL;
    let _ = db.read().wait_for_ready();
    let url = db.read().connection_string();
    sqlx::MssqlPool::connect(&url).await
}

/// A shared connection to the testing Mssql database.
/// Automatically booted and drop as needed
pub fn conn_string() -> String {
    let db = &MSSQL;
    let _ = db.read().wait_for_ready();
    db.read().connection_string().to_string()
}

pub fn init() {
    let db = &MSSQL;
    db.read().is_running();
}

pub fn wait_with_ready() {
    let db = &MSSQL;
    db.read().wait_for_ready().unwrap();
}

pub(crate) struct Mssql {
    port: u32,
    container_id: String,
    password: String,
}

impl Mssql {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let port: u32 = rng.gen_range(20000..49000);

        let password: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        let mut db = Self {
            container_id: String::default(),
            port,
            password,
        };
        eprintln!("Booting Mssql test Environment");
        db.boot().unwrap();
        db
    }

    fn connection_string(&self) -> String {
        format!("mssql://sa:{}@127.0.0.1:{}/", self.password, self.port)
    }

    fn boot(&mut self) -> Result<(), String> {
        let port = format!("127.0.0.1:{}:1433", self.port);
        let env = format!("SA_PASSWORD={}", self.password);
        let output = Command::new("docker")
            .arg("run")
            .arg("--rm")
            .arg("-p")
            .arg(port)
            .arg("--env")
            .arg(env)
            .arg("-d")
            .arg("welds_mssql_testing_db")
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
            .arg("All SQL Seeded")
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
                sleep(ten_millis);
                sleep(ten_millis);
                return Ok(());
            }
            sleep(ten_millis);
        }
    }
}

impl Drop for Mssql {
    fn drop(&mut self) {
        let _ = Command::new("docker")
            .arg("kill")
            .arg(&self.container_id)
            .output();
    }
}
