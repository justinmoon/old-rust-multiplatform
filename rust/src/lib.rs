uniffi::setup_scaffolding!();

mod db;

use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use crossbeam::channel::{unbounded, Receiver, Sender};
use once_cell::sync::OnceCell;

use db::Database;

// globals.rs
static APP: OnceCell<RwLock<App>> = OnceCell::new();
static UPDATER: OnceCell<Updater> = OnceCell::new();

// events.rs
#[derive(uniffi::Enum)]
pub enum Event {
    Increment,
    Decrement,
    TimerStart,
    TimerPause,
    TimerReset,
    SetRoute { route: Route },
}

#[derive(uniffi::Enum)]
pub enum Update {
    CountChanged { count: i32 },
    Timer { state: TimerState },
    // FIXME: https://github.com/mozilla/uniffi-rs/issues/1853
    RouterUpdate { router: Router },
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

#[derive(Clone, uniffi::Record)]
pub struct TimerState {
    elapsed_secs: u32,
    active: bool,
}

impl TimerState {
    pub fn new() -> Self {
        Self {
            elapsed_secs: 0,
            active: false,
        }
    }
}

#[derive(Clone, uniffi::Enum)]
pub enum Route {
    Counter,
    Timer,
}

#[derive(Clone, uniffi::Record)]
pub struct Router {
    route: Route,
}

impl Router {
    pub fn new() -> Self {
        Self {
            route: Route::Counter,
        }
    }
}

#[derive(Clone, uniffi::Record)]
pub struct AppState {
    count: i32,
    timer: TimerState,
    router: Router,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            count: 0,
            timer: TimerState::new(),
            router: Router::new(),
        }
    }
}

#[derive(Clone)]
pub struct App {
    state: Arc<RwLock<AppState>>,
    update_receiver: Arc<Receiver<Update>>,
    handle: Arc<std::thread::JoinHandle<()>>,
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
        let state_clone = state.clone();
        let handle = std::thread::spawn(move || loop {
            std::thread::sleep(Duration::from_secs(1));
            let mut state = state_clone.write().unwrap();
            if !state.timer.active {
                continue;
            }
            state.timer.elapsed_secs += 1;
            Updater::send_update(Update::Timer {
                state: state.timer.clone(),
            });
        });
        let db = Database::new(ffi_app.data_dir.clone()).expect("FIXME");

        // db.update_state("hello, world!").expect("FIXME");
        db.listen_for_updates();

        Self {
            update_receiver: Arc::new(receiver),
            state,
            handle: Arc::new(handle),
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
                // let mut state = state.write().unwrap();
                // state.count += 1;
                // Updater::send_update(Update::CountChanged { count: state.count });
            }
            Event::Decrement => {
                let mut state = state.write().unwrap();
                state.count -= 1;
                Updater::send_update(Update::CountChanged { count: state.count });
            }
            Event::TimerStart => {
                let mut state = state.write().unwrap();
                state.timer.active = true;
                Updater::send_update(Update::Timer {
                    state: state.timer.clone(),
                });
            }
            Event::TimerPause => {
                let mut state = state.write().unwrap();
                state.timer.active = false;
                Updater::send_update(Update::Timer {
                    state: state.timer.clone(),
                });
            }
            Event::TimerReset => {
                let mut state = state.write().unwrap();
                state.timer = TimerState::new();
                Updater::send_update(Update::Timer {
                    state: state.timer.clone(),
                });
            }
            Event::SetRoute { route } => {
                let mut state = state.write().unwrap();
                state.router.route = route;
                Updater::send_update(Update::RouterUpdate {
                    router: state.router.clone(),
                });
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
