use std::time::Instant;

use chip8_core::{
    core::types::C8Byte,
    debugger::{Debugger, DebuggerContext, DebuggerStream},
    drivers::{
        InputInterface, WindowInterface, SCREEN_HEIGHT, SCREEN_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE,
        WINDOW_WIDTH,
    },
    emulator::{EmulationState, Emulator, EmulatorContext},
    errors::CResult,
    peripherals::{
        cartridge::Cartridge,
        input::{InputState, INPUT_STATE_COUNT},
    },
};
pub use pixels;
use pixels::{Pixels, SurfaceTexture};
pub use winit;
use winit::{
    dpi::LogicalSize,
    event::VirtualKeyCode,
    event_loop::{ControlFlow, EventLoop},
    platform::windows::WindowBuilderExtWindows,
    window::Window,
};
use winit_input_helper::WinitInputHelper;

use super::pixels_driver::PixelsRenderDriver;
use crate::UsfxAudioDriver;

/// Window driver for winit
#[derive(Default)]
pub struct WinitWindowDriver;

/// Input driver for winit
pub struct WinitInputDriver {
    helper: WinitInputHelper,
}

impl Default for WinitInputDriver {
    fn default() -> Self {
        Self {
            helper: WinitInputHelper::new(),
        }
    }
}

impl WinitWindowDriver {
    /// Creates new driver.
    pub fn new() -> Self {
        Default::default()
    }

    pub fn create_window(&mut self) -> CResult<(EventLoop<()>, Window)> {
        let event_loop = winit::event_loop::EventLoop::new();
        let sz = LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        let mut wb = winit::window::WindowBuilder::new()
            .with_title(WINDOW_TITLE)
            .with_inner_size(sz)
            .with_min_inner_size(sz);

        if cfg!(windows) {
            // Disable D&D on Windows.
            wb = wb.with_drag_and_drop(false);
        }

        let window = wb.build(&event_loop)?;
        Ok((event_loop, window))
    }
}

impl WinitInputDriver {
    /// Creates new driver.
    pub fn new() -> Self {
        Default::default()
    }

    /// Get helper.
    pub fn helper(&mut self) -> &mut WinitInputHelper {
        &mut self.helper
    }

    fn code_to_key(code: C8Byte) -> VirtualKeyCode {
        match code {
            0x1 => VirtualKeyCode::Key1,
            0x2 => VirtualKeyCode::Key2,
            0x3 => VirtualKeyCode::Key3,
            0xC => VirtualKeyCode::Key4,
            0x4 => VirtualKeyCode::A,
            0x5 => VirtualKeyCode::Z,
            0x6 => VirtualKeyCode::E,
            0xD => VirtualKeyCode::R,
            0x7 => VirtualKeyCode::Q,
            0x8 => VirtualKeyCode::S,
            0x9 => VirtualKeyCode::D,
            0xE => VirtualKeyCode::F,
            0xA => VirtualKeyCode::W,
            0x0 => VirtualKeyCode::X,
            0xB => VirtualKeyCode::C,
            0xF => VirtualKeyCode::V,
            _ => unreachable!(),
        }
    }
}

impl WindowInterface for WinitWindowDriver {
    fn run_emulator(
        &mut self,
        mut emulator: Emulator,
        mut emulator_ctx: EmulatorContext,
        cartridge: Cartridge,
    ) -> CResult {
        let (event_loop, window) = self.create_window()?;

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)?
        };

        let mut input = WinitInputDriver::new();
        emulator
            .cpu
            .drivers
            .set_audio_driver(Box::new(UsfxAudioDriver::default()));

        emulator_ctx.prepare_tracefile(&emulator.cpu.tracefile);

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

            if let winit::event::Event::RedrawRequested(_) = event {
                // Render
                emulator
                    .cpu
                    .peripherals
                    .screen
                    .render_pixels(0, 0, SCREEN_WIDTH as usize, &mut render_driver)
                    .expect("oops");
                pixels.render().expect("Oops");
            }

            if input.helper().update(&event) {
                if input.helper().quit() || input.helper().key_pressed(VirtualKeyCode::Escape) {
                    *control_flow = ControlFlow::Exit;
                    return;
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

    fn run_debugger(
        &mut self,
        debugger: Debugger,
        mut debugger_ctx: DebuggerContext,
        mut emulator: Emulator,
        mut emulator_ctx: EmulatorContext,
        cartridge: Cartridge,
    ) -> CResult {
        let (event_loop, window) = self.create_window()?;

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)?
        };

        let mut input = WinitInputDriver::new();
        emulator
            .cpu
            .drivers
            .set_audio_driver(Box::new(UsfxAudioDriver::default()));

        let mut stream = DebuggerStream::new();
        stream.use_console(true);
        debugger_ctx.is_continuing = true;

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

            if let winit::event::Event::RedrawRequested(_) = event {
                // Render
                emulator
                    .cpu
                    .peripherals
                    .screen
                    .render_pixels(0, 0, SCREEN_WIDTH as usize, &mut render_driver)
                    .expect("oops");
                pixels.render().expect("Oops");
            }

            if input.helper().update(&event) {
                if input.helper().quit() || input.helper().key_pressed(VirtualKeyCode::Escape) {
                    *control_flow = ControlFlow::Exit;
                    return;
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
                    let state = debugger.step(
                        &mut emulator,
                        &mut emulator_ctx,
                        &mut debugger_ctx,
                        &mut stream,
                    );

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
}

impl InputInterface for WinitInputDriver {
    fn update_input_state(&mut self, state: &mut InputState) {
        for key in 0..INPUT_STATE_COUNT {
            let key8 = key as C8Byte;
            let val = Self::code_to_key(key8);

            if self.helper.key_pressed(val) {
                state.press(key8);
            }

            if self.helper.key_released(val) {
                state.release(key8);
            }
        }
    }
}
