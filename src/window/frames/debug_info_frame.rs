//! Debug info frame

use sdl2::rect::Rect;

use crate::emulator::Emulator;
use crate::error::CResult;
use crate::window::draw::{draw_text, DrawContext};
use crate::window::frame::Frame;

/// Debug info frame
pub struct DebugInfoFrame {
    frame: Frame,
}

impl DebugInfoFrame {
    /// Create new frame
    pub fn new(rect: Rect) -> Self {
        Self {
            frame: Frame::new(rect, "DEBUG"),
        }
    }

    /// Render
    pub fn render(&self, emulator: &Emulator, ctx: &mut DrawContext) -> CResult {
        let font = ctx.font_handler.get_or_create_font("default", 8).unwrap();
        let mut output = String::new();

        {
            output.push_str("REGISTERS:");

            for (idx, rgx) in emulator
                .cpu
                .borrow()
                .registers
                .get_registers()
                .iter()
                .enumerate()
            {
                if idx % 5 == 0 {
                    output.push_str("\n");
                }

                output.push_str(&format!("R{:X}={:02X} ", idx, rgx));
            }

            output.push_str(&format!(
                "I={:04X}\n",
                emulator.cpu.borrow().registers.get_i_register()
            ));
        }

        {
            output.push_str("\nSTACK:");

            for (idx, d) in emulator.cpu.borrow().stack.get_data().iter().enumerate() {
                if idx % 7 == 0 {
                    output.push_str("\n");
                }

                output.push_str(&format!("{:04X} ", d));
            }

            output.push_str(&format!(
                "\nPTR={:04X}\n",
                emulator.cpu.borrow().stack.get_pointer()
            ));
        }

        {
            output.push_str("\nTIMERS:\n");
            output.push_str(&format!(
                "DELAY={}\n",
                emulator.cpu.borrow().delay_timer.get_value()
            ));
            output.push_str(&format!(
                "SOUND={}\n",
                emulator.cpu.borrow().sound_timer.get_value()
            ));
        }

        {
            output.push_str("\nINPUT:");

            for (idx, v) in emulator
                .cpu
                .borrow()
                .peripherals
                .input
                .get_data()
                .iter()
                .enumerate()
            {
                if idx % 5 == 0 {
                    output.push_str("\n");
                }

                output.push_str(&format!("K{:X}={:02X} ", idx, v));
            }

            output.push_str(&format!(
                "LK={:02X}\n",
                emulator
                    .cpu
                    .borrow()
                    .peripherals
                    .input
                    .get_last_pressed_key()
            ));
        }

        draw_text(
            ctx.canvas,
            ctx.texture_creator,
            font,
            &output,
            self.frame.rect.x() as u32 + 4,
            self.frame.rect.y() as u32 + 4,
        )?;

        self.frame.render(ctx)?;
        Ok(())
    }
}
