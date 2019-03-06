//! Game scene

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use crate::core::error::CResult;
use crate::emulator::{Emulator, EmulatorContext};
use crate::peripherals::cartridge::Cartridge;
use crate::window::draw::{clear_screen, DrawContext, SCREEN_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::window::frames::game_frame::GameFrame;
use crate::window::frames::keyboard_frame::{KeyboardFrame, KEYBOARD_HEIGHT, KEYBOARD_WIDTH};
use crate::window::frames::status_frame::StatusFrame;
use crate::window::frames::title_frame::TitleFrame;
use crate::window::scene::Scene;
use crate::window::scenemanager::SceneContext;

/// Game scene
pub struct GameScene {
    game_name: Option<String>,
    cartridge: Option<Cartridge>,
    game_frame: GameFrame,
    title_frame: TitleFrame,
    keyboard_frame: KeyboardFrame,
    status_frame: StatusFrame,
    emulator: Emulator,
    emulator_context: EmulatorContext,
}

impl Default for GameScene {
    fn default() -> Self {
        let game_margin = 32;
        let game_x = game_margin;
        let game_y = WINDOW_HEIGHT / 2 - SCREEN_HEIGHT / 2;

        let keyboard_x = WINDOW_WIDTH - game_margin - KEYBOARD_WIDTH;
        let keyboard_y = game_y + (SCREEN_HEIGHT / 2) - (KEYBOARD_HEIGHT / 2);

        Self {
            game_name: None,
            cartridge: None,
            game_frame: GameFrame::new(game_x, game_y),
            title_frame: TitleFrame::new_default("GAME"),
            keyboard_frame: KeyboardFrame::new(keyboard_x, keyboard_y),
            status_frame: StatusFrame::new_default(),
            emulator: Emulator::new(),
            emulator_context: EmulatorContext::new(),
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

        self.title_frame.set_title(&format!("GAME - {}", game));
        self.status_frame.set_status("ESCAPE - Quit");

        self.emulator = Emulator::new();
        self.emulator_context = EmulatorContext::new();
        self.emulator.load_game(self.cartridge.as_ref().unwrap());
    }

    fn destroy(&mut self, _ctx: &mut SceneContext) {}
    fn event(&mut self, _ctx: &mut SceneContext, _e: &Event) {}

    fn render(&mut self, ctx: &mut DrawContext) -> CResult {
        clear_screen(ctx.canvas);

        self.title_frame.render(ctx)?;
        self.status_frame.render(ctx)?;
        self.game_frame.render(&self.emulator, ctx)?;
        self.keyboard_frame.render(&self.emulator, ctx)?;

        Ok(())
    }

    fn update(&mut self, _ctx: &mut SceneContext, pump: &mut EventPump) {
        // Process input
        self.emulator
            .cpu
            .borrow_mut()
            .peripherals
            .input
            .process_input(pump);

        // Step emulation
        self.emulator
            .step(self.cartridge.as_ref().unwrap(), &mut self.emulator_context);
    }

    fn keydown(&mut self, ctx: &mut SceneContext, kc: Keycode) {
        if let Keycode::Escape = kc {
            ctx.set_current_scene("explorer");
        }
    }

    fn keyup(&mut self, _ctx: &mut SceneContext, _kc: Keycode) {}
}
