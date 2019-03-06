//! Window test

use chip8::window::start_window;

#[test]
fn window_test() {
    start_window().unwrap();
}
