use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(data_dir: String) -> Result<Self> {
        let db_path = format!("{}/app_state.db", data_dir);

        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS app_state (
                id INTEGER PRIMARY KEY,
                state TEXT NOT NULL
            )",
            [],
        )?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn update_state(&self, state: &str) -> Result<()> {
        self.conn
            .lock()
            .expect("Failed to lock the connection")
            .execute("INSERT INTO app_state (state) VALUES (?1)", params![state])?;
        Ok(())
    }

    pub fn update_state_loop(&self) {
        let conn_clone = self.conn.clone();
        thread::spawn(move || {
            let mut counter = {
                let conn = conn_clone.lock().expect("Failed to lock the connection");
                let mut stmt = conn
                    .prepare("SELECT state FROM app_state ORDER BY id DESC LIMIT 1")
                    .expect("Failed to prepare statement");
                let mut rows = stmt.query([]).expect("Failed to query");
                if let Some(row) = rows.next().expect("Failed to get next row") {
                    row.get::<_, String>(0)
                        .expect("Failed to get state")
                        .parse::<i32>()
                        .unwrap_or(1)
                } else {
                    1
                }
            };
            loop {
                let state = counter.to_string();
                if let Ok(conn) = conn_clone.lock() {
                    let _ =
                        conn.execute("INSERT INTO app_state (state) VALUES (?1)", params![state]);
                }
                counter += 1;
                thread::sleep(Duration::from_secs(1));
            }
        });
    }
}
