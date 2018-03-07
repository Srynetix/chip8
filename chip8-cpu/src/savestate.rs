//! CHIP-8 save state

use super::input::InputStateData;
use super::memory::Memory;
use super::screen::ScreenData;
use super::registers::Registers;
use super::stack::Stack;
use super::timer::Timer;
use super::cpu::CPU;

/// CHIP-8 save state
#[derive(Clone)]
pub struct SaveState {
    pub input_data: InputStateData,
    pub memory: Memory,
    pub registers: Registers,
    pub screen_data: ScreenData,
    pub stack: Stack,
    pub delay_timer: Timer,
    pub sound_timer: Timer,
    pub instruction_count: usize
}

impl SaveState {

    /// Create save state from CPU
    pub fn save_from_cpu(cpu: &CPU) -> SaveState {
        SaveState {
            input_data: cpu.peripherals.input.data.clone(),
            memory: cpu.peripherals.memory.clone(),
            registers: cpu.registers.clone(),
            screen_data: cpu.peripherals.screen.data.clone(),
            stack: cpu.stack.clone(),
            delay_timer: cpu.delay_timer.clone(),
            sound_timer: cpu.sound_timer.clone(),
            instruction_count: cpu.instruction_count
        }
    }
}