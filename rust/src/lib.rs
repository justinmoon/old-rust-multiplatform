uniffi::setup_scaffolding!();
 
#[uniffi::export]
fn say_hi() -> String {
    "Hello from Rust on Android!".to_string()
}
