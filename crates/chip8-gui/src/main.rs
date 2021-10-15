use chip8_core::{
    drivers::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH},
    errors::CResult,
};
use macroquad::prelude::{clear_background, next_frame, Conf};
use scene::{SceneContext, SceneManager, SceneRunResult};
use scenes::{DebugScene, ExplorerScene, GameScene};

mod draw;
mod frame;
mod frames;
mod scene;
mod scenes;

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

#[macroquad::main(window_conf)]
async fn main() -> CResult {
    let mut ctx = SceneContext::new();
    let mut mgr = SceneManager::new();
    mgr.register_scene("explorer", Box::new(ExplorerScene::new()));
    mgr.register_scene("game", Box::new(GameScene::new()));
    mgr.register_scene("debug", Box::new(DebugScene::new()));
    ctx.set_current_scene("explorer");

    loop {
        clear_background(macroquad::color::BLACK);

        if let SceneRunResult::Stop = mgr.step(&mut ctx) {
            break;
        }

        next_frame().await;
    }

    Ok(())
}
