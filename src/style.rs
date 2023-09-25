
use fltk::enums::{Color, Font};
use fltk::app;

#[derive(Clone)]
pub struct AppStyle {
    pub prog_title_font: Font,
    pub prog_title_color: Color,
    pub prog_title_en_font: Font,
    pub prog_title_en_color: Color,
    pub copyright_font: Font,
    pub copyright_color: Color,
    pub step_title_font: Font,
    pub step_message_font: Font,
    pub step_title_fontsize: i32,
    pub step_message_fontsize: i32,
    pub exit_btn_color: Color,
    pub exit_btn_sel_color: Color,
    pub exit_btn_label_color: Color,
}

impl AppStyle {
    pub fn default(app: &app::App) -> AppStyle {
        app.load_system_fonts();
    
        // for i in app::fonts() {
        //     println!("{}", i);
        // }
    
        let prog_title_en_font = {
                let idx = app::font_index("BArial").unwrap();
                Font::by_index(idx)
        };
    
    
        let font = {
            if let Some(idx) = app::font_index("B微软雅黑") {
                println!("fo");
                Font::by_index(idx)
            } else {
                let idx = app::font_index(" 黑体").unwrap();
                Font::by_index(idx)
            }
        };
    
        let tgu_color = Color::from_rgb(113, 36, 107);
        let darkgrey = Color::from_rgb(80, 80, 80);
        AppStyle {
            prog_title_font: font.clone(),
            prog_title_en_font,
            copyright_font: font.clone(),
            step_title_font: font.clone(),
            step_message_font: font.clone(),
            step_title_fontsize: 18,
            step_message_fontsize: 14,
            prog_title_color: tgu_color,
            prog_title_en_color: tgu_color,
            copyright_color: darkgrey,
            exit_btn_color: Color::from_rgb(200, 20, 20),
            exit_btn_sel_color: Color::from_rgb(255, 20, 20),
            exit_btn_label_color: Color::White,
        }
        
    }
}
