use chip8_core::{drivers::RenderInterface, errors::CResult, peripherals::screen::Color};

pub struct PixelsRenderDriver<'a> {
    frame: &'a mut [u8],
}

impl<'a> PixelsRenderDriver<'a> {
    pub fn new(frame: &'a mut [u8]) -> Self {
        Self { frame }
    }
}

impl<'a> RenderInterface for PixelsRenderDriver<'a> {
    fn render_pixel(
        &mut self,
        origin_x: u32,
        origin_y: u32,
        x: usize,
        y: usize,
        scale: usize,
        color: Color,
        frame_width: usize,
    ) -> CResult {
        let cursor = ((origin_x + (x * scale * 4) as u32)
            + (origin_y + (y * scale * frame_width * 4) as u32)) as usize;
        for l in 0..scale {
            for m in 0..scale {
                let cursor = cursor + l * 4 + (m * frame_width * 4);
                let slice = &mut self.frame[cursor..cursor + 4];
                slice.copy_from_slice(&[color.r, color.g, color.b, color.a]);
            }
        }

        Ok(())
    }
}
