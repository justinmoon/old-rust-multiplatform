use rusqlite::{params, Connection, Result};
use sqlite_watcher::connection::Connection as WatcherConnection;
use sqlite_watcher::watcher::{TableObserver, TableObserverHandle, Watcher};

use std::collections::BTreeSet;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::{Update, Updater};

#[derive(Clone)]
struct Observer {
    name: String,
    tables: Vec<String>,
    // sender: Sender<(String, BTreeSet<String>)>,
}

impl Observer {
    pub fn new(
        name: impl Into<String>,
        tables: Vec<String>,
        // sender: Sender<(String, BTreeSet<String>)>,
    ) -> Observer {
        Self {
            name: name.into(),
            tables,
            // sender,
        }
    }
}

impl TableObserver for Observer {
    fn tables(&self) -> Vec<String> {
        self.tables.clone()
    }

    fn on_tables_changed(&self, tables: &BTreeSet<String>) {
        // panic!("on_tables_changed");
        Updater::send_update(Update::DatabaseUpdate);
        // self.sender
        //     .send((self.name.clone(), tables.clone()))
        //     .unwrap()
    }
}

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<WatcherConnection<Connection>>>,
    // observer: Observer, // FIXME: necessary?
    handle: TableObserverHandle,
}

impl Database {
    pub fn new(data_dir: String) -> Result<Self> {
        let db_path = format!("{}/app_state.db", data_dir);

        let conn = Connection::open(db_path)?;
        let watcher = Watcher::new().unwrap();

        let conn = WatcherConnection::new(conn, Arc::clone(&watcher))?;
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

        // let (sender, receiver) = std::sync::mpsc::channel();
        let observer = Observer::new("observer-1", vec!["app_state".to_owned()]);
        let handle = watcher.add_observer(Box::new(observer)).unwrap();

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            // observer,
            handle,
        })
    }

    // pub fn update_state(&self, state: &str) -> Result<()> {
    //     self.conn
    //         .lock()
    //         .expect("Failed to lock the connection")
    //         .execute("INSERT INTO app_state (state) VALUES (?1)", params![state])?;
    //     Ok(())
    // }

    pub fn increment_state(&self) {
        // panic!("increment_state");
        let mut conn = self.conn.lock().expect("Failed to lock the connection");
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
        conn.sync_watcher_tables().unwrap();
        conn.execute(
            "INSERT INTO app_state (state) VALUES (?1)",
            params![counter.to_string()],
        )
        .unwrap();
        conn.publish_watcher_changes().unwrap();
    }

    pub fn decrement_state(&self) {
        let mut conn = self.conn.lock().expect("Failed to lock the connection");
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
                0
            }
        };
        counter -= 1;
        conn.sync_watcher_tables().unwrap();
        conn.execute(
            "INSERT INTO app_state (state) VALUES (?1)",
            params![counter.to_string()],
        )
        .unwrap();
        conn.publish_watcher_changes().unwrap();
    }
}
