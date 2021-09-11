//! Window module.

pub mod constants;
// pub mod draw;
// pub mod font;
// pub mod frame;
// pub mod frames;
// pub mod scene;
// pub mod scenemanager;
// pub mod scenes;

use std::time::Instant;

use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::VirtualKeyCode;
use winit::event_loop::ControlFlow;
use winit_input_helper::WinitInputHelper;

use crate::core::error::CResult;
use crate::debugger::{Debugger, DebuggerContext, DebuggerStream};
use crate::drivers::WindowInterface;
use crate::emulator::{EmulationState, Emulator, EmulatorContext};
use crate::peripherals::cartridge::Cartridge;

use self::constants::{SCREEN_HEIGHT, SCREEN_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};
// use self::draw::DrawContext;
// use self::font::FontHandler;
// use self::scenemanager::{SceneContext, SceneManager};
// use self::scenes::debug_scene::DebugScene;
// use self::scenes::explorer_scene::ExplorerScene;
// use self::scenes::game_scene::GameScene;
// use self::scenes::home_scene::HomeScene;


/// Start window GUI.
///
/// # Returns
///
/// * Result.
///
pub fn start_window_gui(mut driver: impl WindowInterface) -> CResult {
    driver.run_gui()
}

pub fn start_window_cli(mut driver: impl WindowInterface, emulator: Emulator, emulator_ctx: EmulatorContext, cartridge: Cartridge) -> CResult {
    driver.run_emulator(emulator, emulator_ctx, cartridge)
}

pub fn start_window_cli_debug(mut driver: impl WindowInterface, debugger: Debugger, debugger_ctx: DebuggerContext, emulator: Emulator, emulator_ctx: EmulatorContext, cartridge: Cartridge) -> CResult {
    driver.run_debugger(debugger, debugger_ctx, emulator, emulator_ctx, cartridge)
}

//     // Initialize SDL.
//     let sdl_context = sdl2::init()?;
//     let video_subsys = sdl_context.video()?;
//     let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

//     let window = video_subsys
//         .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
//         .position_centered()
//         .opengl()
//         .build()
//         .map_err(|e| e.to_string())?;

//     let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
//     let texture_creator = canvas.texture_creator();
//     let mut font_handler = FontHandler::new(&ttf_context);

//     // Load a font.
//     let mut assets_dir = env::current_dir()?;
//     assets_dir.push("assets");
//     assets_dir.push("fonts");
//     assets_dir.push("PressStart2P-Regular.ttf");

//     font_handler.register_font_path(&assets_dir, "default");
//     font_handler.create_font("default", 8)?;
//     font_handler.create_font("default", 10)?;

//     canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
//     canvas.clear();
//     canvas.present();

//     let mut draw_context = DrawContext {
//         font_handler: &mut font_handler,
//         texture_creator: &texture_creator,
//         canvas: &mut canvas,
//         video_subsystem: &video_subsys,
//     };

//     let mut event_pump = sdl_context.event_pump()?;
//     let mut scene_ctx = SceneContext::new();
//     let mut scene_manager = SceneManager::new();

//     // Load scenes.
//     let home_scene = Box::new(HomeScene::new());
//     let debug_scene = Box::new(DebugScene::new());
//     let explorer_scene = Box::new(ExplorerScene::new());
//     let game_scene = Box::new(GameScene::new());

//     scene_manager.register_scene("home", home_scene);
//     scene_manager.register_scene("debug", debug_scene);
//     scene_manager.register_scene("explorer", explorer_scene);
//     scene_manager.register_scene("game", game_scene);

//     // Starting scene: home.
//     scene_ctx.set_current_scene("home");

//     scene_manager.run_loop(&mut scene_ctx, &mut draw_context, &mut event_pump);

//     Ok(())
// }
