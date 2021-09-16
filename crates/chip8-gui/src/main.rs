use std::time::Instant;

use chip8_core::{
    drivers::{InputInterface, SCREEN_HEIGHT, SCREEN_WIDTH, WINDOW_TITLE},
    emulator::{EmulationState, Emulator, EmulatorContext},
    errors::CResult,
    peripherals::cartridge::Cartridge,
};
use chip8_drivers::{
    winit_driver::{
        pixels::{Pixels, PixelsContext, SurfaceTexture},
        winit::{
            event::{Event, VirtualKeyCode},
            event_loop::ControlFlow,
        },
        WinitInputDriver,
    },
    PixelsRenderDriver, UsfxAudioDriver, WinitWindowDriver,
};
use wgpu::{CommandEncoder, TextureView};
use wgpu_glyph::{GlyphBrush, GlyphBrushBuilder, Section, Text, ab_glyph};

pub struct FontRenderer {
    staging_belt: wgpu::util::StagingBelt,
    glyph_brush: GlyphBrush<()>
}

impl FontRenderer {
    pub fn new(font_data: &'static [u8], pixels: &Pixels) -> Self {
        let font = ab_glyph::FontArc::try_from_slice(font_data).unwrap();

        Self {
            staging_belt: wgpu::util::StagingBelt::new(1024),
            glyph_brush: GlyphBrushBuilder::using_font(font).build(pixels.device(), pixels.render_texture_format())
        }
    }

    pub fn queue_text(&mut self, section: Section) {
        self.glyph_brush.queue(section);
    }

    pub fn render(&mut self, encoder: &mut CommandEncoder, render_target: &TextureView, context: &PixelsContext, width: u32, height: u32) {
        self.glyph_brush.draw_queued(&context.device, &mut self.staging_belt, encoder, render_target, width, height).unwrap();
        self.staging_belt.finish();
    }
}

const COLOR_PRESSED: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const COLOR_RELEASED: [f32; 4] = [0.25, 0.25, 0.25, 1.25];

pub struct KeyboardRenderer {
    screen_position_x: u32,
    screen_position_y: u32,
    scale: f32,
    keys_config: [[(&'static str, u8); 4]; 4]
}

impl KeyboardRenderer {
    pub fn new(x: u32, y: u32) -> Self {
        let keys_config = [
            [
                (" 1 ", 0x1),
                (" 2 ", 0x2),
                (" 3 ", 0x3),
                (" C ", 0xC),
            ],
            [
                (" 4 ", 0x4),
                (" 5 ", 0x5),
                (" 6 ", 0x6),
                (" D ", 0xD),
            ],
            [
                (" 7 ", 0x7),
                (" 8 ", 0x8),
                (" 9 ", 0x9),
                (" E ", 0xE),
            ],
            [
                (" A ", 0xA),
                (" 0 ", 0x0),
                (" B ", 0xB),
                (" F ", 0xF)
            ]
        ];

        Self {
            screen_position_x: x,
            screen_position_y: y,
            scale: 10.0,
            keys_config
        }
    }

    pub fn update(&self, emulator: &Emulator, font_renderer: &mut FontRenderer) {
        let pressed_color = |key| {
            if emulator.cpu.peripherals.input.get(key) == 0 {
                COLOR_RELEASED
            } else {
                COLOR_PRESSED
            }
        };

        font_renderer.queue_text(Section::default()
            .with_screen_position((self.screen_position_x as f32 + self.scale * 2.0, self.screen_position_y as f32))
            .with_text(
                vec![
                    Text::new("Keyboard")
                        .with_color([1.0, 1.0, 1.0, 1.0])
                        .with_scale(10.0)
                ]
            )
        );

        for (idx, line) in self.keys_config.iter().enumerate() {
            let mut section = Section::default()
                .with_screen_position((self.screen_position_x as f32, self.screen_position_y as f32 + self.scale * 2.0 + self.scale * 2.0 * idx as f32));

            for (txt, val) in line {
                section = section.add_text(
                    Text::new(txt)
                        .with_color(pressed_color(*val))
                        .with_scale(self.scale)
                );
            }

            font_renderer.queue_text(section);
        }
    }
}

fn main() -> CResult {
    let mut driver = WinitWindowDriver::new();
    let (event_loop, window) = driver.create_window()?;

    let cartridge = Cartridge::load_from_path("games/15PUZZLE.ch8")?;
    let mut emulator = Emulator::new();
    emulator
        .cpu
        .drivers
        .set_audio_driver(Box::new(UsfxAudioDriver::default()));
    let mut emulator_ctx = EmulatorContext::new();
    emulator.load_game(&cartridge);

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)?
    };

    let mut input = WinitInputDriver::new();
    let mut font_renderer = FontRenderer::new(include_bytes!(
        "../../../assets/fonts/PressStart2P-Regular.ttf"
    ), &pixels);
    let keyboard_renderer = KeyboardRenderer::new(SCREEN_WIDTH - 125, SCREEN_HEIGHT - 100);

    let mut last_elapsed_time = Instant::now();
    let mut fps_timer = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        let frame_time = last_elapsed_time.elapsed().as_micros();
        last_elapsed_time = Instant::now();
        let mut render_driver = PixelsRenderDriver::new(pixels.get_frame());

        if fps_timer.elapsed().as_millis() > 2000 {
            let frame_time_millis = frame_time as f32 / 1_000.0;
            let frame_time_secs = frame_time_millis as f32 / 1_000.0;
            let fps = (1.0 / frame_time_secs) as u32;

            let title = &format!("[FPS: {} ({} ms)] {}", fps, frame_time_millis, WINDOW_TITLE);
            window.set_title(title);

            fps_timer = Instant::now();
        }

        keyboard_renderer.update(&emulator, &mut font_renderer);

        if let Event::RedrawRequested(_) = event {
            emulator
                .cpu
                .peripherals
                .screen
                .render_pixels(0, 0, SCREEN_WIDTH as usize, &mut render_driver)
                .expect("oops");

            pixels
                .render_with(|encoder, render_target, context| {
                    context.scaling_renderer.render(encoder, render_target);
                    font_renderer.render(encoder, render_target, context, SCREEN_WIDTH, SCREEN_HEIGHT);
                })
                .unwrap();
        }

        if input.helper().update(&event) {
            if input.helper().quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.helper().key_pressed(VirtualKeyCode::Escape) {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.helper().window_resized() {
                pixels.resize_surface(size.width, size.height);
            } else if input.helper().key_pressed(VirtualKeyCode::F5) {
                emulator.reset(&cartridge, &mut emulator_ctx);
                println!("reset");
            } else if input.helper().key_pressed(VirtualKeyCode::F6) {
                emulator.save_state(cartridge.get_title());
                println!("state saved");
            } else if input.helper().key_pressed(VirtualKeyCode::F7) {
                match emulator.load_state(cartridge.get_title()) {
                    Ok(()) => println!("state loaded"),
                    Err(e) => eprintln!("error: {}", e),
                }
            }

            for _ in 0..emulator_ctx.cpu_multiplicator {
                input.update_input_state(&mut emulator.cpu.peripherals.input);

                // Update.
                let state = emulator.step(&mut emulator_ctx);

                // Update state handling
                match state {
                    EmulationState::Quit => {
                        *control_flow = ControlFlow::Exit;
                        break;
                    }
                    EmulationState::WaitForInput => {
                        // Change window title
                        let title = &format!("{} [WAITING FOR INPUT]", WINDOW_TITLE);
                        window.set_title(title);
                        break;
                    }
                    _ => (),
                }
            }

            window.request_redraw();
        }
    });
}
