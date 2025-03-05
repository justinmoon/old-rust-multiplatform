use log;
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::{Event, Update, Updater};

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
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM app_state")?;
        let count: i64 = stmt.query_row([], |row| row.get(0))?;
        if count == 0 {
            conn.execute("INSERT INTO app_state (state) VALUES (?1)", params!["0"])?;
        }
        drop(stmt); // Explicitly drop `stmt` to release the borrow on `conn`
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

    pub fn increment_state(&self) {
        let conn = self.conn.lock().expect("Failed to lock the connection");
        let mut counter = {
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
        counter += 1;
        let _ = conn.execute(
            "INSERT INTO app_state (state) VALUES (?1)",
            params![counter.to_string()],
        );
    }

    // pub fn listen_for_updates(&self, channel: std::sync::mpsc::Sender<String>) {
    pub fn listen_for_updates(&self) {
        // let conn_clone = self.conn.clone();
        // thread::spawn(move || {
        let conn = self.conn.lock().expect("Failed to lock the connection");
        conn.update_hook(Some(
            move |action, db_name: &str, table_name: &str, row_id| {
                let event = format!(
                    "Action: {:?}, Database: {}, Table: {}, Row ID: {}",
                    action, db_name, table_name, row_id
                );
                // if let Err(e) = channel.send(event.clone()) {
                //     eprintln!("Failed to send update event: {}", e);
                // }
                // Updater::send_update(Update::DatabaseUpdate);
                log::info!("{}", event);
                Updater::send_update(Update::DatabaseUpdate);
            },
        ));
        // });
    }
}
