//! # SystemTactics Shared Library
//!
//! Common game logic shared between the main game and development tools.

pub mod colors;
pub mod level;
pub mod rendering;

// Re-export commonly used types
pub use colors::*;
pub use level::*;
pub use rendering::*;
