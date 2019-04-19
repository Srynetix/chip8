//! Window

use std::env;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use crate::core::error::CResult;
use crate::debugger::{Debugger, DebuggerContext, DebuggerState, DebuggerStream};
use crate::emulator::{Emulator, EmulatorContext};
use crate::peripherals::cartridge::Cartridge;

use super::draw::{DrawContext, SCREEN_HEIGHT, SCREEN_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};
use super::font::FontHandler;
use super::scenemanager::{SceneContext, SceneManager};
use super::scenes::debug_scene::DebugScene;
use super::scenes::explorer_scene::ExplorerScene;
use super::scenes::game_scene::GameScene;
use super::scenes::home_scene::HomeScene;

/// Start window GUI
pub fn start_window_gui() -> CResult {
    // Initialize SDL
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsys
        .window("CHIP-8 Emulator GUI", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let mut font_handler = FontHandler::new(&ttf_context);

    // Load a font
    let mut assets_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).to_path_buf();
    assets_dir.push("assets");
    assets_dir.push("fonts");
    assets_dir.push("PressStart2P-Regular.ttf");

    font_handler.register_font_path(&assets_dir, "default");
    font_handler.create_font("default", 8)?;
    font_handler.create_font("default", 10)?;

    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
    canvas.clear();
    canvas.present();

    let mut draw_context = DrawContext {
        font_handler: &mut font_handler,
        texture_creator: &texture_creator,
        canvas: &mut canvas,
        video_subsystem: &video_subsys,
    };

    let mut event_pump = sdl_context.event_pump()?;
    let mut scene_ctx = SceneContext::new();
    let mut scene_manager = SceneManager::new();

    // Load scenes
    let home_scene = Box::new(HomeScene::new());
    let debug_scene = Box::new(DebugScene::new());
    let explorer_scene = Box::new(ExplorerScene::new());
    let game_scene = Box::new(GameScene::new());

    scene_manager.register_scene("home", home_scene);
    scene_manager.register_scene("debug", debug_scene);
    scene_manager.register_scene("explorer", explorer_scene);
    scene_manager.register_scene("game", game_scene);

    // Starting scene: home
    scene_ctx.set_current_scene("home");

    scene_manager.run_loop(&mut scene_ctx, &mut draw_context, &mut event_pump);

    Ok(())
}

/// Start window CLI
pub fn start_window_cli(
    debugger: &mut Debugger,
    debug_ctx: &mut DebuggerContext,
    emulator: &mut Emulator,
    emulator_ctx: &mut EmulatorContext,
    cartridge: &Cartridge,
) -> CResult {
    // Initialize SDL
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let mut event_pump = sdl_context.event_pump()?;

    let window = video_subsys
        .window("CHIP-8 Emulator CLI", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
    canvas.clear();
    canvas.present();

    // Go !
    debug_ctx.is_continuing = true;

    // Create stream
    let mut stream = DebuggerStream::new();

    'running: loop {
        // Event handling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // Render
        canvas.clear();
        emulator
            .cpu
            .borrow_mut()
            .peripherals
            .screen
            .render(0, 0, &mut canvas)?;
        canvas.present();

        // Update
        let state = debugger.step(
            emulator,
            emulator_ctx,
            debug_ctx,
            cartridge,
            &mut event_pump,
            &mut stream,
        );

        if let DebuggerState::Quit = state {
            break 'running;
        }
    }

    Ok(())
}
