use std::sync::OnceLock;

use tokio::runtime::Runtime;
pub fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        Runtime::new()
            .expect("Failed to create runtime")
    })
}