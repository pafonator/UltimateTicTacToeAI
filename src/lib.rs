// Re-export modules so both binaries can use them
pub mod game;  // Your game logic modules
pub mod gui;   // Your GUI modules
pub mod template;  // Your game templates

// Any shared types/functions
pub use gui::ai_worker::{AIWorker, AIRequest, AIResponse};