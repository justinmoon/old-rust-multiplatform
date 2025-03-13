//! Macros for automatic generation of FFI and cross-platform code

/// Registers an application's core components with the framework
/// 
/// This macro generates all the necessary boilerplate code for FFI integration,
/// including global statics, FFI objects with uniffi annotations, and callback interfaces.
///
/// # Example
///
/// ```rust
/// use rust_multiplatform::register_app;
///
/// // Your model, view model, action types...
/// // ...
///
/// register_app!(Model, ViewModel, Action, ModelUpdate);
/// ```
#[macro_export]
macro_rules! register_app {
    ($Model:ident, $ViewModel:ident, $Action:ident, $ModelUpdate:ident) => {
        // 1. Global static definitions for model and view model
        static GLOBAL_MODEL: $crate::once_cell::sync::OnceCell<std::sync::RwLock<$Model>> = 
            $crate::once_cell::sync::OnceCell::new();
        
        static GLOBAL_VIEW_MODEL: $crate::once_cell::sync::OnceCell<$ViewModel> = 
            $crate::once_cell::sync::OnceCell::new();
        
        // 2. Define a wrapper struct for FFI
        #[$crate::uniffi::object]
        pub struct RmpModel {
            pub data_dir: String,
        }
        
        // 3. Implement the FFI interface
        #[$crate::uniffi::export]
        impl RmpModel {
            #[$crate::uniffi::constructor]
            pub fn new(data_dir: String) -> std::sync::Arc<Self> {
                std::sync::Arc::new(Self { data_dir })
            }
            
            pub fn action(&self, action: $Action) {
                // Get the global model and call its action method
                self.get_or_set_global_model()
                    .write()
                    .expect("Failed to acquire write lock on model")
                    .action(action);
            }
            
            pub fn listen_for_model_updates(&self, updater: Box<dyn RmpViewModel>) {
                // Set up the listener
                let model = self.get_or_set_global_model()
                    .read()
                    .expect("Failed to acquire read lock on model");
                    
                $crate::listen_for_model_updates(&*model, updater);
            }
        }
        
        // 4. Helper methods for the FFI object
        impl RmpModel {
            fn get_or_set_global_model(&self) -> &std::sync::RwLock<$Model> {
                GLOBAL_MODEL.get_or_init(|| {
                    // Create a new model
                    let model = <$Model as $crate::traits::RmpAppModel>::create(self.data_dir.clone());
                    std::sync::RwLock::new(model)
                })
            }
        }
        
        // 5. Define the view model callback interface
        #[$crate::uniffi::export(callback_interface)]
        pub trait RmpViewModel: Send + Sync + 'static {
            fn model_update(&self, model_update: $ModelUpdate);
        }
        
        // 6. Extend the ViewModel to integrate with the framework
        impl $ViewModel {
            pub fn init(sender: $crate::crossbeam::channel::Sender<$ModelUpdate>) {
                GLOBAL_VIEW_MODEL.get_or_init(|| $ViewModel(sender));
            }
            
            pub fn model_update(model_update: $ModelUpdate) {
                GLOBAL_VIEW_MODEL
                    .get()
                    .expect("ViewModel is not initialized")
                    .0
                    .send(model_update)
                    .expect("Failed to send model update");
            }
        }
        
        // 7. Set up the uniffi scaffolding
        $crate::uniffi::setup_scaffolding!();
    };
}