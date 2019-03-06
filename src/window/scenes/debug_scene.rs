//! Debug scene

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use crate::core::error::CResult;
use crate::emulator::{Emulator, EmulatorContext};
use crate::peripherals::cartridge::Cartridge;
use crate::window::draw::{
    clear_screen, DrawContext, SCREEN_HEIGHT, SCREEN_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use crate::window::frames::code_frame::CodeFrame;
use crate::window::frames::debug_info_frame::DebugInfoFrame;
use crate::window::frames::game_frame::GameFrame;
use crate::window::frames::shell_frame::ShellFrame;
use crate::window::frames::status_frame::{StatusFrame, STATUS_HEIGHT};
use crate::window::frames::title_frame::{TitleFrame, TITLE_HEIGHT};
use crate::window::scene::Scene;
use crate::window::scenemanager::SceneContext;

const STATUS_TEXT: &str = "\
                           F2 - Cycle\n\
                           F10 - Dump\n\
                           ESCAPE - Quit\
                           ";

/// Debug focus
pub enum DebugFocus {
    /// Main focus
    Main,
    /// Shell focus
    Shell,
}

/// Debug scene
pub struct DebugScene {
    game_name: String,
    cartridge: Cartridge,
    game_frame: GameFrame,
    debug_info_frame: DebugInfoFrame,
    title_frame: TitleFrame,
    code_frame: CodeFrame,
    status_frame: StatusFrame,
    shell_frame: ShellFrame,
    emulator: Emulator,
    emulator_context: EmulatorContext,
    focus: DebugFocus,
}

const SHELL_FRAME_HEIGHT: u32 = 64;
const CODE_FRAME_HEIGHT: u32 =
    WINDOW_HEIGHT - SCREEN_HEIGHT - SHELL_FRAME_HEIGHT - STATUS_HEIGHT - TITLE_HEIGHT;

impl Default for DebugScene {
    fn default() -> Self {
        Self {
            game_frame: GameFrame::new(0, TITLE_HEIGHT),
            debug_info_frame: DebugInfoFrame::new(rectf!(
                SCREEN_WIDTH,
                TITLE_HEIGHT,
                WINDOW_WIDTH - SCREEN_WIDTH,
                SCREEN_HEIGHT
            )),
            code_frame: CodeFrame::new(rectf!(
                0,
                SCREEN_HEIGHT + TITLE_HEIGHT,
                WINDOW_WIDTH,
                CODE_FRAME_HEIGHT
            )),
            title_frame: TitleFrame::new_default("DEBUG"),
            status_frame: StatusFrame::new_default(),
            shell_frame: ShellFrame::new(rectf!(
                0,
                SCREEN_HEIGHT + CODE_FRAME_HEIGHT + TITLE_HEIGHT,
                WINDOW_WIDTH,
                SHELL_FRAME_HEIGHT
            )),
            emulator: Emulator::new(),
            emulator_context: EmulatorContext::new(),
            game_name: String::from("EMPTY"),
            cartridge: Cartridge::new_empty(),
            focus: DebugFocus::Main,
        }
    }
}

impl DebugScene {
    /// Create new scene
    pub fn new() -> Self {
        Default::default()
    }
}

impl Scene for DebugScene {
    fn init(&mut self, ctx: &mut SceneContext) {
        let game = ctx.get_cache_data("selected_game").unwrap();
        let cartridge = Cartridge::load_from_games_directory(&game).expect("bad game name");

        self.game_name = game.clone();
        self.title_frame.set_title(&format!("DEBUG - {}", game));

        {
            let (_code, assembly, verbose) = cartridge.disassemble();
            let mut ptr_value = 0x200;
            for i in 0..assembly.len() {
                let line = format!(
                    "{:04X}| {:3} {:20} ; {}",
                    ptr_value, "", assembly[i], verbose[i]
                );
                self.code_frame.add_text(&line);
                ptr_value += 2;
            }
        }

        self.cartridge = cartridge;

        self.emulator = Emulator::new();
        self.emulator_context = EmulatorContext::new();
        self.emulator.load_game(&self.cartridge);

        self.status_frame.set_status(STATUS_TEXT);
    }

    fn destroy(&mut self, _ctx: &mut SceneContext) {}
    fn event(&mut self, _ctx: &mut SceneContext, e: &Event) {
        if let Event::TextInput { text, .. } = e {
            if let DebugFocus::Shell = self.focus {
                for c in text.chars() {
                    self.shell_frame.add_char(c);
                }
            }
        }
    }

    fn render(&mut self, ctx: &mut DrawContext) -> CResult {
        clear_screen(ctx.canvas);

        self.title_frame.render(ctx)?;
        self.game_frame.render(&self.emulator, ctx)?;
        self.debug_info_frame.render(&self.emulator, ctx)?;
        self.code_frame.render(ctx)?;
        self.shell_frame.render(ctx)?;
        self.status_frame.render(ctx)?;

        Ok(())
    }

    fn keydown(&mut self, ctx: &mut SceneContext, kc: Keycode) {
        match kc {
            Keycode::Escape => {
                ctx.set_current_scene("explorer");
            }
            Keycode::F2 => {
                self.focus = match self.focus {
                    DebugFocus::Main => {
                        println!("Shell mode");
                        self.shell_frame.set_active(true);
                        DebugFocus::Shell
                    }
                    DebugFocus::Shell => {
                        println!("Main mode");
                        self.shell_frame.set_active(false);
                        DebugFocus::Main
                    }
                };
            }
            Keycode::Backspace => {
                if let DebugFocus::Shell = self.focus {
                    self.shell_frame.remove_char();
                }
            }
            Keycode::Return => {
                if let DebugFocus::Shell = self.focus {
                    self.shell_frame.validate();
                }
            }
            Keycode::F10 => {
                let filename = format!("{}.dump", self.cartridge.get_title());
                self.cartridge.write_disassembly_to_file(&filename);

                println!("Cartridge disassembled to {}.", filename);
            }
            _ => {}
        }
    }

    fn keyup(&mut self, _ctx: &mut SceneContext, _kc: Keycode) {}
    fn update(&mut self, _ctx: &mut SceneContext, _pump: &mut EventPump) {}
}
