use std::time::Instant;

use chip8_core::{
    core::types::C8Byte,
    debugger::{Debugger, DebuggerContext, DebuggerStream},
    drivers::{
        InputInterface, RenderInterface, WindowInterface, SCREEN_HEIGHT, SCREEN_WIDTH, WINDOW_TITLE,
    },
    emulator::{EmulationState, Emulator, EmulatorContext},
    errors::CResult,
    peripherals::{
        cartridge::Cartridge,
        input::{InputState, INPUT_STATE_COUNT},
    },
};
use macroquad::prelude::{
    clear_background, draw_text, draw_texture, is_key_pressed, is_key_released, next_frame,
    screen_height, screen_width, Conf, Image, KeyCode, Texture2D,
};

use crate::UsfxAudioDriver;

pub struct MQRenderDriver {
    pub image: Image,
}

#[derive(Default)]
pub struct MQInputDriver;

#[derive(Default)]
pub struct MQWindowDriver;

impl MQWindowDriver {
    pub fn new() -> Self {
        Self::default()
    }
}

impl WindowInterface for MQWindowDriver {
    fn run_emulator(
        &mut self,
        mut emulator: Emulator,
        mut emulator_ctx: EmulatorContext,
        cartridge: Cartridge,
    ) -> CResult {
        let config = Conf {
            window_title: WINDOW_TITLE.into(),
            window_width: SCREEN_WIDTH as i32,
            window_height: SCREEN_HEIGHT as i32,
            window_resizable: false,
            ..Default::default()
        };

        let run = || async move {
            let mut last_elapsed_time = Instant::now();
            let mut fps_timer = Instant::now();
            let mut fps_str = format!("FPS: {} ({} ms)", 0, 0);

            let mut render_driver = MQRenderDriver::new();
            let texture = Texture2D::from_image(&render_driver.image);
            let mut input = MQInputDriver::new();

            emulator
                .cpu
                .drivers
                .set_audio_driver(Box::new(UsfxAudioDriver::default()));

            let origin_x = ((screen_width() - SCREEN_WIDTH as f32) / 2.) as u32;
            let origin_y = ((screen_height() - SCREEN_HEIGHT as f32) / 2.) as u32;

            'mainloop: loop {
                let frame_time = last_elapsed_time.elapsed().as_micros();
                last_elapsed_time = Instant::now();

                clear_background(macroquad::color::BLACK);

                if fps_timer.elapsed().as_millis() > 500 {
                    let frame_time_millis = frame_time as f32 / 1_000.0;
                    let frame_time_secs = frame_time_millis as f32 / 1_000.0;
                    let fps = (1.0 / frame_time_secs) as u32;

                    fps_str = format!("FPS: {} ({} ms)", fps, frame_time_millis);
                    fps_timer = Instant::now();
                }

                // Render
                emulator
                    .cpu
                    .peripherals
                    .screen
                    .render_pixels(
                        origin_x,
                        origin_y,
                        SCREEN_WIDTH as usize,
                        &mut render_driver,
                    )
                    .expect("oops");

                // Input handling
                if is_key_pressed(KeyCode::Escape) {
                    break 'mainloop;
                }

                if is_key_pressed(KeyCode::F5) {
                    emulator.reset(&cartridge, &mut emulator_ctx);
                }

                if is_key_pressed(KeyCode::F6) {
                    emulator.save_state(cartridge.get_title());
                }

                if is_key_pressed(KeyCode::F7) {
                    emulator.load_state(cartridge.get_title()).ok();
                }

                for _ in 0..emulator_ctx.cpu_multiplicator {
                    input.update_input_state(&mut emulator.cpu.peripherals.input);
                    let state = emulator.step(&mut emulator_ctx);

                    match state {
                        EmulationState::Quit => {
                            break 'mainloop;
                        }
                        EmulationState::WaitForInput => {
                            fps_str = "WAITING FOR INPUT".into();
                            break;
                        }
                        _ => (),
                    }
                }

                texture.update(&render_driver.image);
                draw_texture(texture, 0., 0., macroquad::color::WHITE);
                draw_text(&fps_str, 32., 32., 30., macroquad::color::WHITE);
                next_frame().await;
            }
        };

        macroquad::Window::from_config(config, run());

        Ok(())
    }

    fn run_debugger(
        &mut self,
        debugger: Debugger,
        mut debugger_ctx: DebuggerContext,
        mut emulator: Emulator,
        mut emulator_ctx: EmulatorContext,
        cartridge: Cartridge,
    ) -> CResult {
        let config = Conf {
            window_title: WINDOW_TITLE.into(),
            window_width: SCREEN_WIDTH as i32,
            window_height: SCREEN_HEIGHT as i32,
            window_resizable: false,
            ..Default::default()
        };

        let run = || async move {
            let mut last_elapsed_time = Instant::now();
            let mut fps_timer = Instant::now();
            let mut fps_str = format!("FPS: {} ({} ms)", 0, 0);

            let mut render_driver = MQRenderDriver::new();
            let texture = Texture2D::from_image(&render_driver.image);
            let mut input = MQInputDriver::new();

            let mut stream = DebuggerStream::new();
            stream.use_console(true);
            debugger_ctx.is_continuing = true;

            emulator
                .cpu
                .drivers
                .set_audio_driver(Box::new(UsfxAudioDriver::default()));

            let origin_x = ((screen_width() - SCREEN_WIDTH as f32) / 2.) as u32;
            let origin_y = ((screen_height() - SCREEN_HEIGHT as f32) / 2.) as u32;

            'mainloop: loop {
                let frame_time = last_elapsed_time.elapsed().as_micros();
                last_elapsed_time = Instant::now();

                clear_background(macroquad::color::BLACK);

                if fps_timer.elapsed().as_millis() > 500 {
                    let frame_time_millis = frame_time as f32 / 1_000.0;
                    let frame_time_secs = frame_time_millis as f32 / 1_000.0;
                    let fps = (1.0 / frame_time_secs) as u32;

                    fps_str = format!("FPS: {} ({} ms)", fps, frame_time_millis);
                    fps_timer = Instant::now();
                }

                // Render
                emulator
                    .cpu
                    .peripherals
                    .screen
                    .render_pixels(
                        origin_x,
                        origin_y,
                        SCREEN_WIDTH as usize,
                        &mut render_driver,
                    )
                    .expect("oops");

                // Input handling
                if is_key_pressed(KeyCode::Escape) {
                    break 'mainloop;
                }

                if is_key_pressed(KeyCode::F5) {
                    emulator.reset(&cartridge, &mut emulator_ctx);
                }

                if is_key_pressed(KeyCode::F6) {
                    emulator.save_state(cartridge.get_title());
                }

                if is_key_pressed(KeyCode::F7) {
                    emulator.load_state(cartridge.get_title()).ok();
                }

                for _ in 0..emulator_ctx.cpu_multiplicator {
                    input.update_input_state(&mut emulator.cpu.peripherals.input);
                    let state = debugger.step(
                        &mut emulator,
                        &mut emulator_ctx,
                        &mut debugger_ctx,
                        &mut stream,
                    );

                    match state {
                        EmulationState::Quit => {
                            break 'mainloop;
                        }
                        EmulationState::WaitForInput => {
                            fps_str = "WAITING FOR INPUT".into();
                            break;
                        }
                        _ => (),
                    }
                }

                texture.update(&render_driver.image);
                draw_texture(texture, 0., 0., macroquad::color::WHITE);
                draw_text(&fps_str, 32., 32., 30., macroquad::color::WHITE);
                next_frame().await;
            }
        };

        macroquad::Window::from_config(config, run());

        Ok(())
    }
}

impl MQInputDriver {
    pub fn new() -> Self {
        Self::default()
    }

    fn code_to_key(code: C8Byte) -> KeyCode {
        match code {
            0x1 => KeyCode::Key1,
            0x2 => KeyCode::Key2,
            0x3 => KeyCode::Key3,
            0xC => KeyCode::Key4,
            0x4 => KeyCode::A,
            0x5 => KeyCode::Z,
            0x6 => KeyCode::E,
            0xD => KeyCode::R,
            0x7 => KeyCode::Q,
            0x8 => KeyCode::S,
            0x9 => KeyCode::D,
            0xE => KeyCode::F,
            0xA => KeyCode::W,
            0x0 => KeyCode::X,
            0xB => KeyCode::C,
            0xF => KeyCode::V,
            _ => unreachable!(),
        }
    }
}

impl InputInterface for MQInputDriver {
    fn update_input_state(&mut self, state: &mut InputState) {
        for key in 0..INPUT_STATE_COUNT {
            let key8 = key as C8Byte;
            let val = Self::code_to_key(key8);

            if is_key_pressed(val) {
                state.press(key8);
            }

            if is_key_released(val) {
                state.release(key8);
            }
        }
    }
}

impl MQRenderDriver {
    pub fn new() -> Self {
        let w = screen_width() as u16;
        let h = screen_height() as u16;

        Self {
            image: Image::gen_image_color(w, h, macroquad::color::BLACK),
        }
    }
}

impl Default for MQRenderDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderInterface for MQRenderDriver {
    fn render_pixel(
        &mut self,
        origin_x: u32,
        origin_y: u32,
        x: usize,
        y: usize,
        scale: usize,
        color: chip8_core::peripherals::screen::Color,
        _frame_width: usize,
    ) -> CResult {
        for l in 0..scale {
            for m in 0..scale {
                let x = origin_x + l as u32 + x as u32 * scale as u32;
                let y = origin_y + m as u32 + y as u32 * scale as u32;
                self.image.set_pixel(
                    x,
                    y,
                    macroquad::color::Color::from_rgba(color.r, color.g, color.b, color.a),
                );
            }
        }

        Ok(())
    }
}
