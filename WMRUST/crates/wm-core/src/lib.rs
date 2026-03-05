pub mod config;
pub mod enums;
pub mod girl;
pub mod gold;
pub mod item;
pub mod room;
pub mod screen;
pub mod traits;
pub mod xml;

/// Base path to game resources. Override with WM_RESOURCES_PATH env var.
pub fn resources_path() -> std::path::PathBuf {
    std::env::var("WM_RESOURCES_PATH")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("resources"))
}
