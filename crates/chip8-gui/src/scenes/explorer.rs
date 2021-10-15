use chip8_core::{
    core::math::modulo,
    drivers::{WINDOW_HEIGHT, WINDOW_WIDTH},
    peripherals::cartridge::Cartridge,
};
use macroquad::prelude::{is_key_pressed, KeyCode, Rect};

use crate::{
    frames::{ListFrame, ListFrameData, StatusFrame, TitleFrame},
    scene::{Scene, SceneContext},
};

const STATUS_TEXT: &str = "\
                           UP - Move up        F3 - Debug\n\
                           DOWN - Move down    ESC - Quit\n\
                           RETURN - Confirm\
                           ";

/// Explorer scene.
pub struct ExplorerScene {
    list_frame: ListFrame,
    status_frame: StatusFrame,
    title_frame: TitleFrame,
    game_list: Vec<String>,
    game_cursor: i32,
}

impl Default for ExplorerScene {
    fn default() -> Self {
        Self {
            list_frame: ListFrame::new(
                Rect::new(
                    WINDOW_WIDTH as f32 / 4.,
                    WINDOW_HEIGHT as f32 / 4.,
                    WINDOW_WIDTH as f32 / 2.,
                    WINDOW_HEIGHT as f32 / 2.,
                ),
                "GAME LIST",
            ),
            status_frame: StatusFrame::new_default(),
            title_frame: TitleFrame::new("CHIP-8 Emulator - Select a game"),
            game_list: vec![],
            game_cursor: 0,
        }
    }
}

impl ExplorerScene {
    /// Create new scene.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Scene for ExplorerScene {
    fn init(&mut self, _ctx: &mut SceneContext) {
        self.game_list = Cartridge::list_from_games_directory();

        self.status_frame.set_status(STATUS_TEXT);
    }

    fn destroy(&mut self, _ctx: &mut SceneContext) {
        self.game_list.clear();
    }

    fn render(&mut self) {
        let data = ListFrameData {
            cursor: self.game_cursor,
            data: &self.game_list,
        };

        self.title_frame.render();
        self.list_frame.render(&data);
        self.status_frame.render();
    }

    fn update(&mut self, ctx: &mut SceneContext) {
        if is_key_pressed(KeyCode::Up) {
            self.game_cursor = modulo(self.game_cursor - 1, self.game_list.len() as i32)
        } else if is_key_pressed(KeyCode::Down) {
            self.game_cursor = modulo(self.game_cursor + 1, self.game_list.len() as i32)
        } else if is_key_pressed(KeyCode::F3) {
            let mut game_dir = Cartridge::get_games_directory();
            let game = &self.game_list[self.game_cursor as usize];
            game_dir.push(game);

            ctx.set_cache_data(
                "selected_game_path",
                String::from(game_dir.to_str().unwrap()),
            );
            ctx.set_current_scene("debug");
        } else if is_key_pressed(KeyCode::Escape) {
            ctx.quit();
        } else if is_key_pressed(KeyCode::Enter) {
            let mut game_dir = Cartridge::get_games_directory();
            let game = &self.game_list[self.game_cursor as usize];
            game_dir.push(game);

            ctx.set_cache_data(
                "selected_game_path",
                String::from(game_dir.to_str().unwrap()),
            );
            ctx.set_current_scene("game");
        }
    }
}
