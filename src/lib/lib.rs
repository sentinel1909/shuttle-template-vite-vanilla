// src/lib/lib.rs

// module declarations
pub mod service;
pub mod handlers;
pub mod startup;
pub mod telemetry;

// re-exports
pub use service::*;
pub use startup::*;
pub use telemetry::*;
