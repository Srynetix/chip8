use macroquad::prelude::{
    draw_rectangle, draw_rectangle_lines, draw_text_ex, load_ttf_font_from_bytes, Color, Font,
    Rect, TextDimensions, TextParams,
};
use once_cell::sync::Lazy;

static DEFAULT_FONT: Lazy<Font> = Lazy::new(|| {
    load_ttf_font_from_bytes(include_bytes!(
        "../../../assets/fonts/PressStart2P-Regular.ttf"
    ))
    .unwrap()
});

pub fn ui_draw_frame(rect: Rect) {
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., macroquad::color::WHITE);
}

pub fn ui_draw_frame_ex(rect: Rect, color: Color) {
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., color);
}

pub fn ui_draw_text(text: &str, x: f32, y: f32, font_size: u16) {
    ui_draw_text_ex(text, x, y, font_size, macroquad::color::WHITE);
}

pub fn ui_draw_text_ex(text: &str, x: f32, y: f32, font_size: u16, color: Color) {
    _ui_draw_text_multiline(text, x, y, font_size, color)
}

pub fn _ui_draw_text_multiline(text: &str, x: f32, y: f32, font_size: u16, color: Color) {
    let split_text = text.split('\n');
    let cur_x = x;
    let mut cur_y = y;
    let font_height = font_size as f32 + 1.;

    for text in split_text {
        if !text.is_empty() {
            draw_text_ex(
                text,
                cur_x,
                cur_y,
                TextParams {
                    color,
                    font_size,
                    font: *DEFAULT_FONT,
                    ..Default::default()
                },
            )
        }

        cur_y += font_height;
    }
}

pub fn ui_text_size(title: &str, font_size: u16) -> TextDimensions {
    // measure_text(title, None, font_size, 1.)
    return TextDimensions {
        width: title.chars().count() as f32 * font_size as f32,
        height: font_size as f32,
        offset_y: 0.,
    };
}

pub fn ui_draw_fill_rect(rect: Rect, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
}
