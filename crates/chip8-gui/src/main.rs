use std::{path::PathBuf, process};

use argh::FromArgs;
use chip8_core::{
    drivers::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH},
    peripherals::cartridge::Cartridge,
};
use macroquad::prelude::{clear_background, next_frame, Conf};
use scene::{SceneContext, SceneManager, SceneRunResult};
use scenes::{DebugScene, ExplorerScene, GameScene};

mod draw;
mod frame;
mod frames;
mod input;
mod scene;
mod scenes;

/// CHIP-8 Emulator GUI
#[derive(FromArgs)]
struct Args {
    /// game
    #[argh(positional)]
    pub game_path: Option<PathBuf>,

    /// use debug UI
    #[argh(switch)]
    pub debug: bool,
}

fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.into(),
        fullscreen: false,
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

fn main() {
    let args: Args = argh::from_env();
    let s = tracing_subscriber::fmt();
    s.compact().init();

    let amain = || async move {
        let mut ctx = SceneContext::new();
        let mut mgr = SceneManager::new();
        mgr.register_scene("explorer", Box::new(ExplorerScene::new()));
        mgr.register_scene("game", Box::new(GameScene::new()));
        mgr.register_scene("debug", Box::new(DebugScene::new()));

        if let Some(game_path) = args.game_path {
            if let Err(e) = Cartridge::load_from_path(&game_path) {
                eprintln!(
                    "Error while opening cartridge '{}': {}",
                    game_path.display(),
                    e
                );
                process::exit(1);
            }

            ctx.set_cache_data(
                "selected_game_path",
                game_path.to_string_lossy().to_string(),
            );

            if args.debug {
                ctx.set_current_scene("debug")
            } else {
                ctx.set_current_scene("game");
            }
        } else {
            ctx.set_current_scene("explorer");
        }

        loop {
            clear_background(macroquad::color::BLACK);

            if let SceneRunResult::Stop = mgr.step(&mut ctx) {
                break;
            }

            next_frame().await;
        }
    };

    macroquad::Window::from_config(window_conf(), amain());
}
