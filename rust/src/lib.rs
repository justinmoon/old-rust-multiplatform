uniffi::setup_scaffolding!();

mod db;

use std::sync::{Arc, RwLock};

use crossbeam::channel::{unbounded, Receiver, Sender};
use once_cell::sync::OnceCell;

use db::Database;

use rusqlite::{
    types::ValueRef,
    types::{FromSql, ToSql},
    Error, Result,
};

static APP: OnceCell<RwLock<App>> = OnceCell::new();
static UPDATER: OnceCell<Updater> = OnceCell::new();

// TODO: namespace these: counter, router, etc.
#[derive(uniffi::Enum)]
pub enum Event {
    Increment,
    Decrement,
    PushRoute { route: Route },
    PopRoute,
    ResetNavigationStack,
}

#[derive(uniffi::Enum)]
pub enum Update {
    // TODO: include all the update info
    DatabaseUpdate,
}

// FIXME(justin): this is more of an "event bus"
struct Updater(pub Sender<Update>);

impl Updater {
    /// Initialize global instance of the updater with a sender
    pub fn init(sender: Sender<Update>) {
        UPDATER.get_or_init(|| Updater(sender));
    }

    pub fn send_update(update: Update) {
        UPDATER
            .get()
            .expect("updater is not initialized")
            .0
            .send(update)
            .expect("failed to send update");
    }
}

// FIXME(justin): seems like this should be called FFiListener or something like
// that. Maybe the callback should be `handle_update`?
// #[uniffi::export(with_foreign)]
#[uniffi::export(callback_interface)]
pub trait FfiUpdater: Send + Sync + 'static {
    /// Essentially a callback to the frontend
    fn update(&self, update: Update);
}

#[derive(Clone, uniffi::Enum)]
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

#[derive(Clone, uniffi::Record)]
pub struct Router {
    route: Route,
}

impl Router {
    pub fn new() -> Self {
        Self {
            route: Route::Home, // Default to Home screen now
        }
    }
}

#[derive(Clone, uniffi::Record)]
pub struct AppState {
    router: Router,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }
}

#[derive(Clone)]
pub struct App {
    state: Arc<RwLock<AppState>>,
    update_receiver: Arc<Receiver<Update>>,
    db: Database,
}

impl App {
    /// Create a new instance of the app
    pub fn new(ffi_app: &FfiApp) -> Self {
        android_logger::init_once(
            android_logger::Config::default().with_min_level(log::Level::Info),
        );

        let (sender, receiver): (Sender<Update>, Receiver<Update>) = unbounded();
        Updater::init(sender);
        let state = Arc::new(RwLock::new(AppState::new()));

        // FIXME: put this elsewhere ...
        // And perhapse this should just emit events which handle_event listens for?
        let db = Database::new(ffi_app.data_dir.clone()).expect("FIXME");

        Self {
            update_receiver: Arc::new(receiver),
            state,
            db,
        }
    }

    /// Fetch global instance of the app, or create one if it doesn't exist
    pub fn global(ffi_app: &FfiApp) -> &'static RwLock<App> {
        APP.get_or_init(|| RwLock::new(App::new(ffi_app)))
    }

    /// Handle event received from frontend
    pub fn handle_event(&self, event: Event) {
        // Handle event
        let state = self.state.clone();
        match event {
            Event::Increment => {
                self.db.increment_state();
            }
            Event::Decrement => {
                self.db.decrement_state();
            }
            Event::PushRoute { route } => {
                self.db.push_route(&route).expect("Failed to push route");
            }
            Event::PopRoute => {
                self.db.pop_route().expect("Failed to pop route");
            }
            Event::ResetNavigationStack => {
                self.db
                    .reset_navigation_stack()
                    .expect("Failed to reset navigation stack");
            }
        }
    }

    // FIXME(justin): if we only need one subscriber, we could move this logic to
    // the constructor
    pub fn listen_for_updates(&self, updater: Box<dyn FfiUpdater>) {
        let update_receiver = self.update_receiver.clone();
        std::thread::spawn(move || {
            while let Ok(field) = update_receiver.recv() {
                updater.update(field);
            }
        });
    }

    pub fn get_state(&self) -> AppState {
        self.state.read().unwrap().clone()
    }
}

/// Representation of our app over FFI. Essentially a wrapper of [`App`].
#[derive(uniffi::Object)]
pub struct FfiApp {
    // FIXME: this is database path currently, not actually data dir
    data_dir: String,
}

#[uniffi::export]
impl FfiApp {
    /// FFI constructor which wraps in an Arc
    #[uniffi::constructor]
    pub fn new(data_dir: String) -> Arc<Self> {
        Arc::new(Self { data_dir })
    }

    /// Frontend calls this method to send events to the rust application logic
    pub fn dispatch(&self, event: Event) {
        // FIXME: this won't be able to handle concurrent events ...
        self.inner().write().expect("fixme").handle_event(event);
    }

    pub fn listen_for_updates(&self, updater: Box<dyn FfiUpdater>) {
        self.inner()
            .read()
            .expect("fixme")
            .listen_for_updates(updater);
    }

    pub fn get_state(&self) -> AppState {
        self.inner().read().unwrap().get_state()
    }
}

impl FfiApp {
    /// Fetch global instance of the app, or create one if it doesn't exist
    fn inner(&self) -> &RwLock<App> {
        App::global(self)
    }
}
