use std::time::Instant;

use chip8_core::{
    drivers::{InputInterface, SCREEN_HEIGHT, SCREEN_WIDTH, WINDOW_TITLE},
    emulator::{EmulationState, Emulator, EmulatorContext},
    errors::CResult,
    peripherals::cartridge::Cartridge,
};
use chip8_drivers::{MQInputDriver, MQRenderDriver, UsfxAudioDriver};
use macroquad::prelude::{
    clear_background, draw_text, draw_texture, is_key_pressed, next_frame, screen_height,
    screen_width, Conf, KeyCode, Texture2D,
};

// const COLOR_PRESSED: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
// const COLOR_RELEASED: [f32; 4] = [0.25, 0.25, 0.25, 1.25];

// pub struct DebugRenderer {
//     screen_position_x: u32,
//     screen_position_y: u32,
//     scale: f32,
// }

// impl DebugRenderer {
//     pub fn new(x: u32, y: u32) -> Self {
//         Self {
//             screen_position_x: x,
//             screen_position_y: y,
//             scale: 12.0,
//         }
//     }

//     pub fn update(&self, emulator: &Emulator) {
//         todo!()
//     }
// }

// pub struct KeyboardRenderer {
//     screen_position_x: u32,
//     screen_position_y: u32,
//     scale: f32,
//     keys_config: [[(&'static str, u8); 4]; 4],
// }

// impl KeyboardRenderer {
//     pub fn new(x: u32, y: u32) -> Self {
//         let keys_config = [
//             [(" 1 ", 0x1), (" 2 ", 0x2), (" 3 ", 0x3), (" C ", 0xC)],
//             [(" 4 ", 0x4), (" 5 ", 0x5), (" 6 ", 0x6), (" D ", 0xD)],
//             [(" 7 ", 0x7), (" 8 ", 0x8), (" 9 ", 0x9), (" E ", 0xE)],
//             [(" A ", 0xA), (" 0 ", 0x0), (" B ", 0xB), (" F ", 0xF)],
//         ];

//         Self {
//             screen_position_x: x,
//             screen_position_y: y,
//             scale: 10.0,
//             keys_config,
//         }
//     }

//     pub fn update(&self, emulator: &Emulator) {
// let pressed_color = |key| {
//     if emulator.cpu.peripherals.input.get(key) == 0 {
//         COLOR_RELEASED
//     } else {
//         COLOR_PRESSED
//     }
// };

// font_renderer.queue_text(
//     Section::default()
//         .with_screen_position((
//             self.screen_position_x as f32 + self.scale * 2.0,
//             self.screen_position_y as f32,
//         ))
//         .with_text(vec![Text::new("Keyboard")
//             .with_color([1.0, 1.0, 1.0, 1.0])
//             .with_scale(10.0)]),
// );

// for (idx, line) in self.keys_config.iter().enumerate() {
//     let mut section = Section::default().with_screen_position((
//         self.screen_position_x as f32,
//         self.screen_position_y as f32 + self.scale * 2.0 + self.scale * 2.0 * idx as f32,
//     ));

//     for (txt, val) in line {
//         section = section.add_text(
//             Text::new(txt)
//                 .with_color(pressed_color(*val))
//                 .with_scale(self.scale),
//         );
//     }

//     font_renderer.queue_text(section);
// }
//     }
// }

fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.into(),
        fullscreen: false,
        window_width: 1024,
        window_height: 768,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> CResult {
    let cartridge = Cartridge::load_from_path("games/15PUZZLE.ch8")?;
    let mut emulator = Emulator::new();
    emulator
        .cpu
        .drivers
        .set_audio_driver(Box::new(UsfxAudioDriver::default()));
    let mut emulator_ctx = EmulatorContext::new();
    emulator.load_game(&cartridge);

    let mut last_elapsed_time = Instant::now();
    let mut fps_timer = Instant::now();
    let mut fps_str = format!("FPS: {} ({} ms)", 0, 0);

    let mut render_driver = MQRenderDriver::new();
    let texture = Texture2D::from_image(&render_driver.image);
    let mut input = MQInputDriver::new();

    let origin_x = ((screen_width() - SCREEN_WIDTH as f32) / 2.) as u32;
    let origin_y = ((screen_height() - SCREEN_HEIGHT as f32) / 2.) as u32;

    'mainloop: loop {
        let frame_time = last_elapsed_time.elapsed().as_micros();
        last_elapsed_time = Instant::now();

        clear_background(macroquad::color::BLACK);

        if fps_timer.elapsed().as_millis() > 500 {
            let frame_time_millis = frame_time as f32 / 1_000.0;
            let frame_time_secs = frame_time_millis as f32 / 1_000.0;
            let fps = (1.0 / frame_time_secs) as u32;

            fps_str = format!("FPS: {} ({} ms)", fps, frame_time_millis);
            fps_timer = Instant::now();
        }

        // Render
        emulator
            .cpu
            .peripherals
            .screen
            .render_pixels(
                origin_x,
                origin_y,
                SCREEN_WIDTH as usize,
                &mut render_driver,
            )
            .expect("oops");

        // Input handling
        if is_key_pressed(KeyCode::Escape) {
            break 'mainloop;
        }

        if is_key_pressed(KeyCode::F5) {
            emulator.reset(&cartridge, &mut emulator_ctx);
        }

        if is_key_pressed(KeyCode::F6) {
            emulator.save_state(cartridge.get_title());
        }

        if is_key_pressed(KeyCode::F7) {
            emulator.load_state(cartridge.get_title()).ok();
        }

        for _ in 0..emulator_ctx.cpu_multiplicator {
            input.update_input_state(&mut emulator.cpu.peripherals.input);
            let state = emulator.step(&mut emulator_ctx);

            match state {
                EmulationState::Quit => {
                    break 'mainloop;
                }
                EmulationState::WaitForInput => {
                    fps_str = "WAITING FOR INPUT".into();
                    break;
                }
                _ => (),
            }
        }

        texture.update(&render_driver.image);
        draw_texture(texture, 0., 0., macroquad::color::WHITE);
        draw_text(&fps_str, 32., 32., 30., macroquad::color::WHITE);
        next_frame().await;
    }

    Ok(())
}
