use crate::Database;
use once_cell::sync::OnceCell;
use rusqlite::{
    types::ValueRef,
    types::{FromSql, ToSql},
    Result,
};
use std::sync::RwLock;

// Access the global DATABASE from database.rs
extern crate self as counter;
use crate::database::DATABASE;

/// Route enum represents the different screens in the application
#[derive(uniffi::Enum, Debug, Clone, PartialEq)]
pub enum Route {
    Counter,
    Timer,
    // New routes for the app
    Home,
    Mint,
    MintAmount,
    MintConfirm,
    Melt,
    MeltConfirm,
    TransactionHistory,
    Success,
    Error,
}

impl ToSql for Route {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput> {
        let value = match self {
            Route::Counter => "counter",
            Route::Timer => "timer",
            Route::Home => "home",
            Route::Mint => "mint",
            Route::MintAmount => "mint_amount",
            Route::MintConfirm => "mint_confirm",
            Route::Melt => "melt",
            Route::MeltConfirm => "melt_confirm",
            Route::TransactionHistory => "transaction_history",
            Route::Success => "success",
            Route::Error => "error",
        };
        Ok(rusqlite::types::ToSqlOutput::from(value))
    }
}

impl FromSql for Route {
    fn column_result(value: ValueRef) -> rusqlite::types::FromSqlResult<Self> {
        value.as_str().and_then(|s| match s {
            "counter" => Ok(Route::Counter),
            "timer" => Ok(Route::Timer),
            "home" => Ok(Route::Home),
            "mint" => Ok(Route::Mint),
            "mint_amount" => Ok(Route::MintAmount),
            "mint_confirm" => Ok(Route::MintConfirm),
            "melt" => Ok(Route::Melt),
            "melt_confirm" => Ok(Route::MeltConfirm),
            "transaction_history" => Ok(Route::TransactionHistory),
            "success" => Ok(Route::Success),
            "error" => Ok(Route::Error),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        })
    }
}

/// Router struct that represents the navigation history
/// A Record type for UniFFI to pass across FFI
#[derive(Clone, Debug, uniffi::Record)]
pub struct Router {
    pub routes: Vec<Route>,
}

impl Router {
    /// Create a new empty Router
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /// Create a Router by querying the database
    /// Note: This is a regular function, not exposed via FFI
    /// Use FfiDatabase.get_router() instead for FFI
    pub fn from_database() -> Self {
        // Get the database from the global DATABASE variable
        if let Some(db_lock) = DATABASE.get() {
            if let Ok(db) = db_lock.read() {
                if let Ok(routes) = get_routes_from_db(&db) {
                    return Self { routes };
                }
            }
        }
        // Return empty router if anything fails
        Self::new()
    }

    /// Get the current route (top of the stack)
    pub fn current_route(&self) -> Option<Route> {
        self.routes.last().cloned()
    }
}

// Helper function used by FfiDatabase to get routes data
pub fn get_routes_from_db(db: &Database) -> Result<Vec<Route>> {
    // Execute the SQL query to get all routes
    let query = "SELECT route_name FROM navigation_stack ORDER BY id";
    let mut routes = Vec::new();

    // Use the database connection to execute the query
    let conn = db.get_connection();
    let mut stmt = conn.prepare(query)?;
    let rows = stmt.query_map([], |row| row.get::<_, Route>(0))?;

    // Collect all routes into a vector
    for route_result in rows {
        if let Ok(route) = route_result {
            routes.push(route);
        }
    }

    Ok(routes)
}
