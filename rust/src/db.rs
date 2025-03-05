use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};

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
            .expect("FIXME")
            .execute("INSERT INTO app_state (state) VALUES (?1)", params![state])?;
        Ok(())
    }
}
