//! Explorer scene

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use crate::cartridge::Cartridge;
use crate::error::CResult;
use crate::math::modulo;
use crate::window::draw::{clear_screen, DrawContext, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::window::scene::Scene;
use crate::window::scenemanager::SceneContext;

use crate::window::frames::list_frame::{ListFrame, ListFrameData};
use crate::window::frames::status_frame::StatusFrame;
use crate::window::frames::title_frame::TitleFrame;

const STATUS_TEXT: &str = "\
                           UP - Move up        F3 - Debug\n\
                           DOWN - Move down\n\
                           RETURN - Confirm\n\
                           ESCAPE - Quit\
                           ";

/// Explorer scene
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
                rectf!(
                    WINDOW_WIDTH / 4,
                    WINDOW_HEIGHT / 4,
                    WINDOW_WIDTH / 2,
                    WINDOW_HEIGHT / 2
                ),
                "GAME LIST",
            ),
            status_frame: StatusFrame::new_default(),
            title_frame: TitleFrame::new_default("LOAD GAME - Select a game"),
            game_list: vec![],
            game_cursor: 0,
        }
    }
}

impl ExplorerScene {
    /// Create new scene
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

    fn event(&mut self, _ctx: &mut SceneContext, _e: &Event) {}

    fn render(&mut self, ctx: &mut DrawContext) -> CResult {
        clear_screen(ctx.canvas);

        let data = ListFrameData {
            cursor: self.game_cursor,
            data: &self.game_list,
        };

        self.title_frame.render(ctx)?;
        self.list_frame.render(ctx, &data)?;
        self.status_frame.render(ctx)?;

        Ok(())
    }

    fn update(&mut self, _ctx: &mut SceneContext, _event_pump: &mut EventPump) {}

    fn keydown(&mut self, ctx: &mut SceneContext, kc: Keycode) {
        match kc {
            Keycode::Up => {
                self.game_cursor = modulo(self.game_cursor - 1, self.game_list.len() as i32)
            }
            Keycode::Down => {
                self.game_cursor = modulo(self.game_cursor + 1, self.game_list.len() as i32)
            }
            Keycode::F3 => {
                let game = &self.game_list[self.game_cursor as usize];
                ctx.set_cache_data("selected_game", game.clone());
                ctx.set_current_scene("debug");
            }
            Keycode::Escape => ctx.set_current_scene("home"),
            Keycode::Return => {
                let game = &self.game_list[self.game_cursor as usize];
                ctx.set_cache_data("selected_game", game.clone());
                ctx.set_current_scene("game");
            }
            _ => {}
        }
    }

    fn keyup(&mut self, _ctx: &mut SceneContext, _kc: Keycode) {}
}
