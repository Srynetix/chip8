//! Game scene

use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use crate::cartridge::Cartridge;
use crate::error::CResult;
use crate::window::draw::{clear_screen, DrawContext};
use crate::window::scene::Scene;
use crate::window::scenemanager::SceneContext;

/// Game scene
pub struct GameScene {
    game_name: Option<String>,
    cartridge: Option<Cartridge>,
}

impl Default for GameScene {
    fn default() -> Self {
        Self {
            game_name: None,
            cartridge: None,
        }
    }
}

impl GameScene {
    /// Create new scene
    pub fn new() -> Self {
        Default::default()
    }
}

impl Scene for GameScene {
    fn init(&mut self, ctx: &mut SceneContext) {
        let game = ctx.get_cache_data("selected_game").unwrap();
        self.game_name = Some(game.clone());
        self.cartridge = Some(Cartridge::load_from_games_directory(&game).expect("bad game name"));
    }

    fn destroy(&mut self, _ctx: &mut SceneContext) {}
    fn render(&mut self, ctx: &mut DrawContext) -> CResult {
        clear_screen(ctx.canvas);

        Ok(())
    }
    fn input(&mut self, _ctx: &mut SceneContext, _pump: &mut EventPump) {}
    fn keydown(&mut self, ctx: &mut SceneContext, kc: Keycode) {
        if let Keycode::Escape = kc {
            ctx.set_current_scene("explorer");
        }
    }
    fn keyup(&mut self, _ctx: &mut SceneContext, _kc: Keycode) {}
}
