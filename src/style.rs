use fltk::enums::{Color, Font};

pub const COLOR_TGU: Color = Color::from_rgb(113, 36, 107);
pub const COLOR_DARKGREY: Color = Color::from_rgb(80, 80, 80);
pub const COLOR_GREY: Color = Color::from_rgb(200, 200, 200);
pub const COLOR_MESSAGE: Color = Color::from_rgb(10, 10, 10);

pub fn set_gui_font() {
    let windir = match std::env::var("WINDIR") {
        Ok(value) => value,
        Err(_) => "C:\\Windows".to_string(),
    };

    let font_dir = std::path::Path::new(&windir).join("Fonts");
    if !font_dir.exists() {
        log::error!("no font_dir: {}", font_dir.to_string_lossy());
        return;
    }

    let font_msyh_file = font_dir.join("msyh.ttc");
    if font_msyh_file.exists() {
        if let Ok(font) = Font::load_font(&font_msyh_file) {
            Font::set_font(Font::Helvetica, &font);
            log::info!("set font: {}", font_msyh_file.to_string_lossy())
        } else {
            log::error!("can not load font: {}", font_msyh_file.to_string_lossy())
        }
    } else {
        log::error!("no font: {}", font_dir.to_string_lossy());
    }
}
