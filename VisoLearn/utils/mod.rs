// Export utility modules
pub mod file_operations;
pub mod state_management;
pub mod visualization;

// Re-export commonly used functions from utility modules
pub use file_operations::{save_project, load_project, export_image};
pub use state_management::{SessionState, create_session, save_session};
pub use visualization::{render_histogram, display_metrics};