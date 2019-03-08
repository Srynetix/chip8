//! Window module

mod _window;
pub mod draw;
pub mod font;
pub mod frame;
pub mod frames;
pub mod scene;
pub mod scenemanager;
pub mod scenes;

pub use _window::{start_window_cli, start_window_gui};
