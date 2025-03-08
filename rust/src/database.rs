use rusqlite::{params, Connection, Result};
use sqlite_watcher::connection::Connection as WatcherConnection;
use sqlite_watcher::watcher::{TableObserver, Watcher};

use once_cell::sync::OnceCell;
use std::collections::BTreeSet;
use std::sync::{Arc, Mutex, RwLock};

use crate::navigation::Router;
use crate::{Route, Update, Updater};

// Global static DATABASE instance
pub static DATABASE: OnceCell<RwLock<Database>> = OnceCell::new();

#[derive(Clone)]
struct Observer {
    tables: Vec<String>,
}

impl Observer {
    pub fn new(tables: Vec<String>) -> Observer {
        Self { tables }
    }
}

impl TableObserver for Observer {
    fn tables(&self) -> Vec<String> {
        self.tables.clone()
    }

    fn on_tables_changed(&self, tables: &BTreeSet<String>) {
        // If navigation stack changed, send detailed update
        if tables.contains("navigation_stack") {
            // Use Router::get() to simplify getting router data
            let router = Router::get();
            let current_route = router.current_route();

            // Send detailed update with router data
            Updater::send_update(Update::RouterUpdate {
                router,
                current_route,
            });
        }
    }
}

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<WatcherConnection<Connection>>>,
}

impl Database {
    pub fn new(data_dir: String) -> Result<Self> {
        let db_path = format!("{}/app_state.db", data_dir);

        let conn = Connection::open(db_path)?;
        let watcher = Watcher::new().unwrap();

        let conn = WatcherConnection::new(conn, Arc::clone(&watcher))?;

        // Create app_state table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS app_state (
                id INTEGER PRIMARY KEY,
                state TEXT NOT NULL
            )",
            [],
        )?;

        // Create navigation_stack table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS navigation_stack (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                route_name TEXT NOT NULL
            )",
            [],
        )?;

        let mut stmt = conn.prepare("SELECT COUNT(*) FROM app_state")?;
        let count: i64 = stmt.query_row([], |row| row.get(0))?;
        if count == 0 {
            conn.execute("INSERT INTO app_state (state) VALUES (?1)", params!["0"])?;
        }
        drop(stmt); // Explicitly drop `stmt` to release the borrow on `conn`

        let observer = Observer::new(vec!["app_state".to_owned(), "navigation_stack".to_owned()]);
        watcher.add_observer(Box::new(observer)).unwrap();

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Get the global database instance
    /// This assumes DATABASE has been initialized by FfiApp::new
    /// and unwraps the RwLock for convenience
    pub fn global() -> Database {
        DATABASE
            .get()
            .expect("DATABASE not initialized. Call FfiApp::new first")
            .read()
            .expect("Failed to get read lock on DATABASE")
            .clone()
    }

    fn execute(&self, statement: &str, params: &[&dyn rusqlite::ToSql]) -> Result<()> {
        let mut conn = self.conn.lock().expect("FIXME");
        conn.sync_watcher_tables()?;
        conn.execute(statement, params)?;
        conn.publish_watcher_changes()?;
        Ok(())
    }

    pub fn push_route(&self, route: &Route) -> Result<()> {
        self.execute(
            "INSERT INTO navigation_stack (route_name) VALUES (?1)",
            &[route],
        )
    }

    pub fn pop_route(&self) -> Result<()> {
        self.execute(
            "DELETE FROM navigation_stack WHERE id = (SELECT MAX(id) FROM navigation_stack)",
            &[],
        )
    }

    pub fn reset_navigation_stack(&self) -> Result<()> {
        self.execute("DELETE FROM navigation_stack", &[])
    }

    /// Reset the router - alias for reset_navigation_stack
    pub fn reset_router(&self) -> Result<()> {
        self.reset_navigation_stack()
    }

    /// Get a reference to the connection
    /// This is used by the NavigationStack to query routes
    pub fn get_connection(&self) -> std::sync::MutexGuard<WatcherConnection<Connection>> {
        self.conn.lock().expect("Failed to lock the connection")
    }
}

/// Representation of our database over FFI. Wrapper for Database.
#[derive(uniffi::Object)]
pub struct FfiDatabase {
    // Path to database file
    db_path: String,
}

#[uniffi::export]
impl FfiDatabase {
    /// FFI constructor which wraps in an Arc
    #[uniffi::constructor]
    pub fn new(db_path: String) -> Arc<Self> {
        Arc::new(Self { db_path })
    }

    /// Get the router
    pub fn get_router(&self) -> Router {
        Router::get()
    }

    /// Get just the routes from the router
    pub fn get_routes(&self) -> Vec<Route> {
        Router::get().routes
    }

    /// Get the current route (or None if router is empty)
    pub fn get_current_route(&self) -> Option<Route> {
        Router::get().current_route()
    }

    /// Push a route onto the router
    pub fn push_route(&self, route: Route) {
        self.get_database()
            .push_route(&route)
            .expect("Failed to push route");
    }

    /// Pop the top route from the router
    pub fn pop_route(&self) {
        self.get_database()
            .pop_route()
            .expect("Failed to pop route");
    }

    /// Reset the router
    pub fn reset_router(&self) {
        self.get_database()
            .reset_navigation_stack()
            .expect("Failed to reset router");
    }
}

impl FfiDatabase {
    /// Get the Database instance (creates one if it doesn't exist)
    fn get_database(&self) -> Database {
        DATABASE
            .get_or_init(|| {
                let db =
                    Database::new(self.db_path.clone()).expect("Failed to initialize database");
                RwLock::new(db)
            })
            .read()
            .expect("Failed to read database")
            .clone()
    }
}
