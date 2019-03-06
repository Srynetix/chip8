//! Window

use std::env;
use std::path::Path;

use sdl2::pixels::Color;

use crate::core::error::CResult;

use super::draw::{DrawContext, WINDOW_HEIGHT, WINDOW_WIDTH};
use super::font::FontHandler;
use super::scenemanager::{SceneContext, SceneManager};
use super::scenes::debug_scene::DebugScene;
use super::scenes::explorer_scene::ExplorerScene;
use super::scenes::game_scene::GameScene;
use super::scenes::home_scene::HomeScene;

/// Start window
pub fn start_window() -> CResult {
    // Load cartridge
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsys
        .window("CHIP-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
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
