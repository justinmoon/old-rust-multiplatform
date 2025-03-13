use crossbeam::channel::Sender;
use once_cell::sync::OnceCell;

// Global static UPDATER instance
static GLOBAL_VIEW_MODEL: OnceCell<ViewModel> = OnceCell::new();

// FIXME: rename this notification
#[derive(uniffi::Enum)]
pub enum ModelUpdate {}

// FIXME(justin): this is more of an "event bus"
#[derive(Clone)]
pub struct ViewModel(pub Sender<ModelUpdate>);

impl ViewModel {
    /// Initialize global instance of the updater with a sender
    pub fn init(sender: Sender<ModelUpdate>) {
        GLOBAL_VIEW_MODEL.get_or_init(|| ViewModel(sender));
    }

    pub fn dispatch(update: ModelUpdate) {
        GLOBAL_VIEW_MODEL
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
pub trait FfiViewModel: Send + Sync + 'static {
    /// Essentially a callback to the frontend
    fn dispatch(&self, update: ModelUpdate);
}
