use std::path::Path;

use chip8_core::{
    drivers::{InputInterface, SCREEN_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH},
    emulator::{Emulator, EmulatorContext},
    peripherals::cartridge::Cartridge,
};
use chip8_drivers::MQInputDriver;
use macroquad::prelude::{is_key_pressed, KeyCode};

use crate::{
    frames::{GameFrame, KeyboardFrame, StatusFrame, TitleFrame, KEYBOARD_HEIGHT, KEYBOARD_WIDTH},
    scene::{Scene, SceneContext},
};

/// Game scene.
pub struct GameScene {
    game_name: String,
    cartridge: Cartridge,
    game_frame: GameFrame,
    title_frame: TitleFrame,
    keyboard_frame: KeyboardFrame,
    status_frame: StatusFrame,
    emulator: Emulator,
    emulator_context: EmulatorContext,
    input_driver: MQInputDriver,
}

impl Default for GameScene {
    fn default() -> Self {
        let game_margin = 32;
        let game_x = game_margin;
        let game_y = WINDOW_HEIGHT / 2 - SCREEN_HEIGHT / 2;

        let keyboard_x = WINDOW_WIDTH - game_margin - KEYBOARD_WIDTH;
        let keyboard_y = game_y + (SCREEN_HEIGHT / 2) - (KEYBOARD_HEIGHT / 2);

        Self {
            game_name: String::from("EMPTY"),
            cartridge: Cartridge::new_empty(),
            game_frame: GameFrame::new(game_x, game_y),
            title_frame: TitleFrame::new("GAME"),
            keyboard_frame: KeyboardFrame::new(keyboard_x, keyboard_y),
            status_frame: StatusFrame::new_default(),
            emulator: Emulator::new(),
            emulator_context: EmulatorContext::new(),
            input_driver: MQInputDriver::new(),
        }
    }
}

impl GameScene {
    /// Create new scene.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Scene for GameScene {
    fn init(&mut self, ctx: &mut SceneContext) {
        let game_path = ctx.get_cache_data("selected_game_path").unwrap();

        self.game_name = Cartridge::get_game_name(Path::new(&game_path));
        self.cartridge = Cartridge::load_from_path(&game_path).expect("bad game name");

        self.title_frame
            .set_title(&format!("GAME - {}", self.game_name));
        self.status_frame
            .set_status("F5 - Reset          ESC - Back\nF6 - Save state\nF7 - Load state");

        self.emulator = Emulator::new();
        self.emulator_context = EmulatorContext::new();
        self.emulator.load_game(&self.cartridge);

        // Prepare tracefile.
        self.emulator_context
            .prepare_tracefile(&self.emulator.cpu.tracefile);
    }

    fn destroy(&mut self, _ctx: &mut SceneContext) {}

    fn render(&mut self) {
        self.title_frame.render();
        self.status_frame.render();
        self.game_frame.render(&mut self.emulator);
        self.keyboard_frame.render(&self.emulator);
    }

    fn update(&mut self, ctx: &mut SceneContext) {
        if is_key_pressed(KeyCode::Escape) {
            ctx.set_current_scene("explorer");
        } else if is_key_pressed(KeyCode::F5) {
            self.emulator
                .reset(&self.cartridge, &mut self.emulator_context);
        } else if is_key_pressed(KeyCode::F6) {
            self.emulator.save_state(&self.game_name);
        } else if is_key_pressed(KeyCode::F7) {
            self.emulator.load_state(&self.game_name).ok();
        }

        for _ in 0..self.emulator_context.cpu_multiplicator {
            self.input_driver
                .update_input_state(&mut self.emulator.cpu.peripherals.input);
            self.emulator.step(&mut self.emulator_context);
        }
    }
}
