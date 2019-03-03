//! Window test

use chip8::window::emulator_window;

#[test]
fn window_test() {
    emulator_window().unwrap();
}
