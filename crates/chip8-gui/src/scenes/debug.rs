//! Debug scene.

use std::path::{Path, PathBuf};

use chip8_core::{
    debugger::{Command, Debugger, DebuggerContext, DebuggerStream},
    drivers::{InputInterface, SCREEN_HEIGHT, SCREEN_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH},
    emulator::{Emulator, EmulatorContext},
    peripherals::{cartridge::Cartridge, memory::INITIAL_MEMORY_POINTER},
};
use chip8_drivers::{MQAudioDriver, MQInputDriver};
use macroquad::prelude::{get_char_pressed, is_key_pressed, KeyCode, Rect};

use crate::{
    frames::{
        CodeFrame, DebugInfoFrame, GameFrame, MemoryFrame, ShellFrame, StatusFrame, TitleFrame,
        STATUS_HEIGHT, TITLE_HEIGHT,
    },
    scene::{Scene, SceneContext},
};

const STATUS_TEXT: &str = "\
                           F2 - Shell          F4 - Step\n\
                           F3 - Memory         F5 - Continue\n\
                           F10 - Dump          F6 - Pause\n\
                           ESC - Back to game list\
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
    input_driver: MQInputDriver,
}

const CODE_FRAME_HEIGHT: u32 = WINDOW_HEIGHT - SCREEN_HEIGHT - STATUS_HEIGHT - TITLE_HEIGHT;

impl Default for DebugScene {
    fn default() -> Self {
        Self {
            game_frame: GameFrame::new(0, TITLE_HEIGHT),
            debug_info_frame: DebugInfoFrame::new(Rect::new(
                SCREEN_WIDTH as f32,
                TITLE_HEIGHT as f32,
                WINDOW_WIDTH as f32 - SCREEN_WIDTH as f32,
                SCREEN_HEIGHT as f32,
            )),
            code_frame: CodeFrame::new(Rect::new(
                0.,
                SCREEN_HEIGHT as f32 + TITLE_HEIGHT as f32,
                WINDOW_WIDTH as f32,
                CODE_FRAME_HEIGHT as f32,
            )),
            title_frame: TitleFrame::new("DEBUG"),
            status_frame: StatusFrame::new_default(),
            shell_frame: ShellFrame::new(Rect::new(
                WINDOW_WIDTH as f32 / 4.,
                WINDOW_HEIGHT as f32 / 4.,
                WINDOW_WIDTH as f32 / 2.,
                WINDOW_HEIGHT as f32 / 2.,
            )),
            memory_frame: MemoryFrame::new(Rect::new(
                64.,
                64.,
                WINDOW_WIDTH as f32 - 64. * 2.,
                WINDOW_HEIGHT as f32 - 64. * 2.,
            )),
            emulator: Emulator::new(),
            emulator_context: EmulatorContext::new(),
            debugger: Debugger::new(),
            debugger_context: DebuggerContext::new(),
            debugger_stream: DebuggerStream::new(),
            game_name: String::from("EMPTY"),
            cartridge: Cartridge::new_empty(),
            focus: DebugFocus::Main,
            input_driver: MQInputDriver::new(),
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
        self.emulator
            .cpu
            .drivers
            .set_audio_driver(Box::new(MQAudioDriver::default()));

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

    fn render(&mut self) {
        self.title_frame.render();
        self.game_frame.render(&mut self.emulator);
        self.debug_info_frame
            .render(&self.emulator, &self.debugger_context);
        self.code_frame.render(&self.debugger_context);
        self.status_frame.render();

        match self.focus {
            DebugFocus::Shell => self.shell_frame.render(&self.debugger_stream),
            DebugFocus::Memory => self.memory_frame.render(&self.emulator),
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut SceneContext) {
        if is_key_pressed(KeyCode::Escape) {
            ctx.set_current_scene("explorer");
        } else if is_key_pressed(KeyCode::F2) {
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
        } else if is_key_pressed(KeyCode::F3) {
            self.focus = match self.focus {
                DebugFocus::Main | DebugFocus::Shell => DebugFocus::Memory,
                DebugFocus::Memory => DebugFocus::Main,
            };
        } else if is_key_pressed(KeyCode::F4) {
            self.debugger.handle_command(
                &self.emulator.cpu,
                &mut self.debugger_context,
                &mut self.debugger_stream,
                Command::Step,
            );
        } else if is_key_pressed(KeyCode::F5) {
            self.debugger.handle_command(
                &self.emulator.cpu,
                &mut self.debugger_context,
                &mut self.debugger_stream,
                Command::Continue,
            );
        } else if is_key_pressed(KeyCode::F6) {
            self.debugger_context.is_continuing = false;
        } else if is_key_pressed(KeyCode::Backspace) {
            if let DebugFocus::Shell = self.focus {
                self.shell_frame.remove_char();
            }
        } else if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
            if let DebugFocus::Shell = self.focus {
                let cmd_str = self.shell_frame.validate();
                self.debugger_stream
                    .writeln_stdout(format!("> {}", cmd_str));

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
        } else if is_key_pressed(KeyCode::F10) {
            let filename = format!("{}.dump", self.cartridge.get_title());
            let path = PathBuf::from(filename);
            self.cartridge.write_disassembly_to_file(Some(path));
        } else if let Some(c) = get_char_pressed() {
            if let DebugFocus::Shell = self.focus {
                self.shell_frame.add_char(c);
            }
        }

        for _ in 0..self.emulator.cpu.speed_multiplicator {
            self.input_driver
                .update_input_state(&mut self.emulator.cpu.peripherals.input);
            self.debugger.step(
                &mut self.emulator,
                &mut self.emulator_context,
                &mut self.debugger_context,
                &mut self.debugger_stream,
            );
        }
    }
}
