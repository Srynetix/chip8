//! Debug info frame.

use chip8_core::{debugger::DebuggerContext, emulator::Emulator};
use macroquad::prelude::Rect;

use crate::{draw::ui_draw_text, frame::Frame};

/// Debug info frame.
pub struct DebugInfoFrame {
    frame: Frame,
}

impl DebugInfoFrame {
    /// Create new frame.
    pub fn new(rect: Rect) -> Self {
        Self {
            frame: Frame::new(rect, "DEBUG"),
        }
    }

    /// Render.
    pub fn render(&self, emulator: &Emulator, debug_ctx: &DebuggerContext) {
        let font_size = 12;
        let mut output = String::new();

        {
            output.push_str("REGISTERS:");

            for (idx, rgx) in emulator.cpu.registers.get_registers().iter().enumerate() {
                if idx % 5 == 0 {
                    output.push('\n');
                }

                output.push_str(&format!("V{:X}={:02X} ", idx, rgx));
            }

            output.push_str(&format!(
                "I={:04X}\n",
                emulator.cpu.registers.get_i_register()
            ));
        }

        {
            output.push_str("\nSTACK:");

            for (idx, d) in emulator.cpu.stack.get_data().iter().enumerate() {
                if idx % 7 == 0 {
                    output.push('\n');
                }

                output.push_str(&format!("{:04X} ", d));
            }

            output.push_str(&format!("\nPTR={:04X}\n", emulator.cpu.stack.get_pointer()));
        }

        {
            output.push_str("\nTIMERS:\n");
            output.push_str(&format!("DELAY={}\n", emulator.cpu.delay_timer.get_value()));
            output.push_str(&format!("SOUND={}\n", emulator.cpu.sound_timer.get_value()));
            output.push_str(&format!("SYNC={}\n", emulator.cpu.sync_timer.get_value()));
        }

        {
            output.push_str("\nINPUT:");

            for (idx, v) in emulator.cpu.peripherals.input.get_data().iter().enumerate() {
                if idx % 5 == 0 {
                    output.push('\n');
                }

                output.push_str(&format!("K{:X}={:02X} ", idx, v));
            }

            output.push_str(&format!(
                "LK={:02X} ",
                emulator.cpu.peripherals.input.get_last_pressed_key()
            ));

            output.push_str(&format!(
                "WAIT={}\n",
                if emulator.cpu.peripherals.input.is_locked() {
                    1
                } else {
                    0
                }
            ));
        }

        {
            let emulation_state = if debug_ctx.is_paused() {
                "PAUSED"
            } else {
                "RUNNING"
            };

            output.push_str(&format!("\nEmulation state: {}", emulation_state));
        }

        ui_draw_text(
            &output,
            self.frame.rect.x + 4.,
            self.frame.rect.y + 4.,
            font_size,
        );

        self.frame.render();
    }
}
