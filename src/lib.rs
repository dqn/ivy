// Many public APIs are not used internally but are provided for external use.
#![allow(dead_code)]

pub mod flowchart;
pub mod hotreload;
pub mod i18n;
pub mod input;
pub mod modding;
pub mod platform;
pub mod runtime;
pub mod scenario;
pub mod types;
pub mod video;

// Re-export accessibility types
pub mod accessibility;
pub use accessibility::{SelfVoicing, SelfVoicingMode};
