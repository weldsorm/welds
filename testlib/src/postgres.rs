use rand::{distributions::Alphanumeric, Rng};
use static_init::dynamic;
use std::process::{Command, Stdio};
use std::thread::sleep; // 0.8

#[dynamic(drop)]
static mut POSTGRES: Postgres = Postgres::new();

/// A shared connection to the testing Postgres database.
/// Automatically booted and drop as needed
pub async fn conn() -> Result<sqlx::PgPool, sqlx::Error> {
    let pg = &POSTGRES;
    let _ = pg.read().wait_for_ready();
    let url = pg.read().connection_string();
    sqlx::PgPool::connect(&url).await
}

/// A shared connection to the testing Postgres database.
/// Automatically booted and drop as needed
pub fn conn_string() -> String {
    let pg = &POSTGRES;
    let _ = pg.read().wait_for_ready();
    pg.read().connection_string().to_string()
}

pub(crate) fn init() {
    let pg = &POSTGRES;
    pg.read().is_running();
}

pub(crate) fn wait_with_ready() {
    let pg = &POSTGRES;
    pg.read().wait_for_ready().unwrap();
}

pub(crate) struct Postgres {
    port: u32,
    password: String,
    container_id: String,
}

impl Postgres {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let port: u32 = rng.gen_range(20000..49000);

        let password: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        let mut pg = Self {
            container_id: String::default(),
            port,
            password,
        };
        eprintln!("Booting Postgres test Environment");
        pg.boot().unwrap();
        pg
    }

    fn connection_string(&self) -> String {
        format!(
            "postgresql://postgres:{}@127.0.0.1:{}",
            self.password, self.port
        )
    }

    fn boot(&mut self) -> Result<(), String> {
        let port = format!("127.0.0.1:{}:5432", self.port);
        let env = format!("POSTGRES_PASSWORD={}", self.password);
        let output = Command::new("docker")
            .arg("run")
            .arg("--rm")
            .arg("-p")
            .arg(port)
            .arg("--env")
            .arg(env)
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
            .arg("checkpoint complete: wrote")
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
                // HACK: PG says it is ready before it really is :/
                sleep(ten_millis);
                sleep(ten_millis);
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
