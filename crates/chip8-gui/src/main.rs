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
            window::Window,
        },
        WinitInputDriver,
    },
    PixelsRenderDriver, UsfxAudioDriver, WinitWindowDriver,
};
use egui::{ClippedMesh, CtxRef, FontDefinitions};
use egui_wgpu_backend::{
    wgpu::{CommandEncoder, TextureView},
    BackendError, RenderPass, ScreenDescriptor,
};
use egui_winit_platform::{Platform, PlatformDescriptor};

pub struct Gui {
    start_time: Instant,
    platform: Platform,
    screen_descriptor: ScreenDescriptor,
    rpass: RenderPass,
    paint_jobs: Vec<ClippedMesh>,
    about_window_open: bool,
    explorer_window_open: bool,
    game_list: Vec<String>,
    selected_game: Option<String>,
    will_update_game: bool,
}

impl Gui {
    pub fn new(width: u32, height: u32, scale_factor: f64, pixels: &Pixels) -> Self {
        let platform = Platform::new(PlatformDescriptor {
            physical_width: width,
            physical_height: height,
            scale_factor,
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });
        let screen_descriptor = ScreenDescriptor {
            physical_width: width,
            physical_height: height,
            scale_factor: scale_factor as f32,
        };
        let rpass = RenderPass::new(pixels.device(), pixels.render_texture_format(), 1);

        Self {
            start_time: Instant::now(),
            platform,
            screen_descriptor,
            rpass,
            paint_jobs: Vec::new(),
            about_window_open: false,
            explorer_window_open: false,
            game_list: Cartridge::list_from_games_directory(),
            selected_game: None,
            will_update_game: false,
        }
    }

    pub fn handle_event(&mut self, event: &Event<'_, ()>) {
        self.platform.handle_event(event)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.screen_descriptor.physical_width = width;
            self.screen_descriptor.physical_height = height;
        }
    }

    pub fn scale_factor(&mut self, scale_factor: f64) {
        self.screen_descriptor.scale_factor = scale_factor as f32;
    }

    pub fn prepare(&mut self, window: &Window) {
        self.platform
            .update_time(self.start_time.elapsed().as_secs_f64());

        self.platform.begin_frame();
        self.ui(&self.platform.context());

        let (_output, paint_commands) = self.platform.end_frame(Some(window));
        self.paint_jobs = self.platform.context().tessellate(paint_commands);
    }

    fn ui(&mut self, ctx: &CtxRef) {
        let games = self.game_list.clone();
        let mut selected_game = self.selected_game.clone();
        let mut will_update_game = false;

        egui::TopBottomPanel::top("menubar_container").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Explorer...").clicked() {
                        self.explorer_window_open = true;
                    }
                    if ui.button("About...").clicked() {
                        self.about_window_open = true;
                    }
                });
            })
        });

        egui::Window::new("About")
            .resizable(false)
            .open(&mut self.about_window_open)
            .show(ctx, |ui| {
                ui.label("CHIP-8 Emulator");
                ui.separator();
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x /= 2.0;
                    ui.label("Source code");
                    ui.hyperlink("https://github.com/Srynetix/chip8");
                });
            });

        egui::Window::new("Explorer")
            // .scroll(true)
            .resizable(false)
            .open(&mut self.explorer_window_open)
            .show(ctx, |ui| {
                ui.label("Select a cartridge to load");
                ui.separator();

                egui::ComboBox::from_label("Select game")
                    .selected_text(if let Some(game) = selected_game.as_ref() {
                        game
                    } else {
                        ""
                    })
                    .width(400.0)
                    .show_ui(ui, |ui| {
                        for game in games {
                            ui.selectable_value(&mut selected_game, Some(game.clone()), game);
                        }
                    });

                let btn = egui::Button::new("Load game").enabled(selected_game.is_some());
                if ui.add(btn).clicked() {
                    will_update_game = true;
                }
            });

        self.will_update_game = will_update_game;
        self.selected_game = selected_game;
    }

    pub fn render(
        &mut self,
        encoder: &mut CommandEncoder,
        render_target: &TextureView,
        context: &PixelsContext,
    ) -> Result<(), BackendError> {
        self.rpass.update_texture(
            &context.device,
            &context.queue,
            &self.platform.context().texture(),
        );
        self.rpass
            .update_user_textures(&context.device, &context.queue);
        self.rpass.update_buffers(
            &context.device,
            &context.queue,
            &self.paint_jobs,
            &self.screen_descriptor,
        );
        self.rpass.execute(
            encoder,
            render_target,
            &self.paint_jobs,
            &self.screen_descriptor,
            None,
        )
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

    let (mut pixels, mut gui) = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)?;
        let gui = Gui::new(window_size.width, window_size.height, scale_factor, &pixels);

        (pixels, gui)
    };

    let mut input = WinitInputDriver::new();

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

        gui.handle_event(&event);

        if gui.will_update_game {
            let game_dir = Cartridge::get_games_directory();
            let cart_path = game_dir.join(gui.selected_game.as_ref().unwrap());
            let cartridge = Cartridge::load_from_path(&cart_path).unwrap();
            emulator.reset(&cartridge, &mut emulator_ctx);
        }

        if let Event::RedrawRequested(_) = event {
            gui.prepare(&window);
            emulator
                .cpu
                .peripherals
                .screen
                .render_pixels(0, 0, SCREEN_WIDTH as usize, &mut render_driver)
                .expect("oops");

            pixels
                .render_with(|encoder, render_target, context| {
                    context.scaling_renderer.render(encoder, render_target);
                    gui.render(encoder, render_target, context).unwrap();
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

            if let Some(scale_factor) = input.helper().scale_factor() {
                gui.scale_factor(scale_factor);
            }

            if let Some(size) = input.helper().window_resized() {
                pixels.resize_surface(size.width, size.height);
                gui.resize(size.width, size.height);
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
