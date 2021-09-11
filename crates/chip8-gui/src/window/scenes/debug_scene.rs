//! Debug scene.

use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use crate::core::error::CResult;
use crate::debugger::{Command, Debugger, DebuggerContext, DebuggerStream};
use crate::emulator::{EmulationState, Emulator, EmulatorContext};
use crate::peripherals::cartridge::Cartridge;
use crate::peripherals::memory::INITIAL_MEMORY_POINTER;
use crate::rectf;
use crate::window::draw::{
    clear_screen, DrawContext, SCREEN_HEIGHT, SCREEN_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use crate::window::frames::code_frame::CodeFrame;
use crate::window::frames::debug_info_frame::DebugInfoFrame;
use crate::window::frames::game_frame::GameFrame;
use crate::window::frames::memory_frame::MemoryFrame;
use crate::window::frames::shell_frame::ShellFrame;
use crate::window::frames::status_frame::{StatusFrame, STATUS_HEIGHT};
use crate::window::frames::title_frame::{TitleFrame, TITLE_HEIGHT};
use crate::window::scene::Scene;
use crate::window::scenemanager::SceneContext;

const STATUS_TEXT: &str = "\
                           F2 - Shell                 F4 - Step\n\
                           F3 - Memory                F5 - Continue\n\
                           F10 - Dump                 F6 - Pause\n\
                           ESCAPE - Quit\
                           ";

/// Debug focus.
pub enum DebugFocus {
    /// Main focus.
    Main,
    /// Shell focus.
    Shell,
    /// Memory focus.
    Memory,
}

/// Debug scene.
pub struct DebugScene {
    game_name: String,
    cartridge: Cartridge,
    game_frame: GameFrame,
    debug_info_frame: DebugInfoFrame,
    title_frame: TitleFrame,
    code_frame: CodeFrame,
    status_frame: StatusFrame,
    shell_frame: ShellFrame,
    memory_frame: MemoryFrame,
    debugger: Debugger,
    debugger_context: DebuggerContext,
    debugger_stream: DebuggerStream,
    emulator: Emulator,
    emulator_context: EmulatorContext,
    focus: DebugFocus,
}

const CODE_FRAME_HEIGHT: u32 = WINDOW_HEIGHT - SCREEN_HEIGHT - STATUS_HEIGHT - TITLE_HEIGHT;

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
                WINDOW_WIDTH / 4,
                WINDOW_HEIGHT / 4,
                WINDOW_WIDTH / 2,
                WINDOW_HEIGHT / 2
            )),
            memory_frame: MemoryFrame::new(rectf!(32, 32, WINDOW_WIDTH - 64, WINDOW_HEIGHT - 64)),
            emulator: Emulator::new(),
            emulator_context: EmulatorContext::new(),
            debugger: Debugger::new(),
            debugger_context: DebuggerContext::new(),
            debugger_stream: DebuggerStream::new(),
            game_name: String::from("EMPTY"),
            cartridge: Cartridge::new_empty(),
            focus: DebugFocus::Main,
        }
    }
}

impl DebugScene {
    /// Create new scene.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Scene for DebugScene {
    fn init(&mut self, ctx: &mut SceneContext) {
        let game_path = ctx.get_cache_data("selected_game_path").unwrap();
        let cartridge = Cartridge::load_from_path(&game_path).expect("bad game name");

        self.game_name = Cartridge::get_game_name(Path::new(&game_path));
        self.title_frame
            .set_title(&format!("DEBUG - {}", self.game_name));

        {
            let (_code, assembly, verbose) = cartridge.disassemble();
            let mut ptr_value = INITIAL_MEMORY_POINTER;
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

        self.debugger = Debugger::new();
        self.debugger_context = DebuggerContext::new();
        self.debugger_context.set_manual();
        self.debugger_context.set_address(INITIAL_MEMORY_POINTER);
        self.debugger_stream = DebuggerStream::new();

        self.status_frame.set_status(STATUS_TEXT);
    }

    fn destroy(&mut self, _ctx: &mut SceneContext) {
        self.code_frame.reset();
        self.shell_frame.reset();

        self.focus = DebugFocus::Main;
    }

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
        self.game_frame.render(&mut self.emulator, ctx)?;
        self.debug_info_frame
            .render(&self.emulator, &self.debugger_context, ctx)?;
        self.code_frame.render(&self.debugger_context, ctx)?;
        self.status_frame.render(ctx)?;

        match self.focus {
            DebugFocus::Shell => self.shell_frame.render(ctx, &self.debugger_stream)?,
            DebugFocus::Memory => self.memory_frame.render(&self.emulator, ctx)?,
            _ => {}
        }

        Ok(())
    }

    fn keydown(&mut self, ctx: &mut SceneContext, kc: Keycode) {
        match kc {
            Keycode::Escape => {
                ctx.set_current_scene("explorer");
            }
            Keycode::F2 => {
                self.focus = match self.focus {
                    DebugFocus::Main | DebugFocus::Memory => {
                        self.shell_frame.set_active(true);
                        DebugFocus::Shell
                    }
                    DebugFocus::Shell => {
                        self.shell_frame.set_active(false);
                        DebugFocus::Main
                    }
                };
            }
            Keycode::F3 => {
                self.focus = match self.focus {
                    DebugFocus::Main | DebugFocus::Shell => DebugFocus::Memory,
                    DebugFocus::Memory => DebugFocus::Main,
                };
            }
            Keycode::F4 => {
                // Step.
                self.debugger.handle_command(
                    &self.emulator.cpu,
                    &mut self.debugger_context,
                    &mut self.debugger_stream,
                    Command::Step,
                );
            }
            Keycode::F5 => {
                // Continue.
                self.debugger.handle_command(
                    &self.emulator.cpu,
                    &mut self.debugger_context,
                    &mut self.debugger_stream,
                    Command::Continue,
                );
            }
            Keycode::F6 => {
                // Pause.
                self.debugger_context.is_continuing = false;
            }
            Keycode::Backspace => {
                if let DebugFocus::Shell = self.focus {
                    self.shell_frame.remove_char();
                }
            }
            Keycode::Return => {
                if let DebugFocus::Shell = self.focus {
                    let cmd_str = self.shell_frame.validate();
                    let cmd = self
                        .debugger
                        .read_command(&cmd_str, &mut self.debugger_stream);
                    if let Some(cmd) = cmd {
                        self.debugger.handle_command(
                            &self.emulator.cpu,
                            &mut self.debugger_context,
                            &mut self.debugger_stream,
                            cmd,
                        );
                    }
                }
            }
            Keycode::F10 => {
                let filename = format!("{}.dump", self.cartridge.get_title());
                self.cartridge.write_disassembly_to_file(&filename);

                println!("cartridge disassembled to {}.", filename);
            }
            _ => {}
        }
    }

    fn keyup(&mut self, _ctx: &mut SceneContext, _kc: Keycode) {}
    fn update(&mut self, ctx: &mut SceneContext, pump: &mut EventPump) {
        // let state = self.debugger.step(
        //     &mut self.emulator,
        //     &mut self.emulator_context,
        //     &mut self.debugger_context,
        //     pump,
        //     &mut self.debugger_stream,
        // );
        let state = EmulationState::Normal;

        if let EmulationState::Quit = state {
            ctx.set_current_scene("explorer");
        }
    }
}
