//! Window module

pub mod font;
pub mod frame;
pub mod scene;
pub mod scenemanager;

#[macro_use]
pub mod draw;

pub mod scenes;

use self::draw::{DrawContext, WINDOW_HEIGHT, WINDOW_WIDTH};
use self::font::FontHandler;
use self::scenemanager::{SceneContext, SceneManager};
use self::scenes::{DebugScene, HomeScene};
use super::cartridge::Cartridge;

use std::env;
use std::error::Error;
use std::path::Path;

use sdl2::pixels::Color;

/// Emulator window test
pub fn emulator_window() -> Result<(), Box<dyn Error>> {
    // Load cartridge
    let cartridge = Cartridge::load_from_games_directory("TEST/BC_test.ch8")?;

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
    };

    let mut event_pump = sdl_context.event_pump()?;
    let mut scene_ctx = SceneContext::new();
    let mut scene_manager = SceneManager::new();

    // Load scene
    let home_scene = Box::new(HomeScene::new());
    let mut debug_scene = Box::new(DebugScene::new());
    debug_scene.load_cartridge_dump(&cartridge);

    scene_manager.register_scene("home", home_scene);
    scene_manager.register_scene("debug", debug_scene);

    // Starting scene: home
    scene_ctx.set_current_scene("home");

    scene_manager.run_loop(&mut scene_ctx, &mut draw_context, &mut event_pump);

    Ok(())
}
