use crossbeam::channel::Sender;
use once_cell::sync::OnceCell;

// Global static UPDATER instance
static UPDATER: OnceCell<Updater> = OnceCell::new();

// FIXME: rename this notification
#[derive(uniffi::Enum)]
pub enum Update {
    // TODO: include all the update info
    DatabaseUpdate,
}

// FIXME(justin): this is more of an "event bus"
#[derive(Clone)]
pub struct Updater(pub Sender<Update>);

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
#[uniffi::export(callback_interface)]
pub trait FfiUpdater: Send + Sync + 'static {
    /// Essentially a callback to the frontend
    fn update(&self, update: Update);
}
