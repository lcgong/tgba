use fltk::{
    app::font_index,
    enums::{Color, Font},
};

pub struct AppStyle {
    pub font_bold_zh: Font,
    pub font_bold_en: Font,
    pub font_zh: Font,
    pub tgu_color: Color,
    pub darkgrey: Color,
}

impl Default for AppStyle {
    fn default() -> Self {
        // let fonts = fltk::app::fonts();
        // for f in fonts {
        //     println!("font: {}", f);
        // }
        let font_bold_en = { Font::by_index(font_index("BArial").unwrap()) };
        let font_en = { Font::by_index(font_index(" Arial").unwrap()) };
        let font_bold_zh = {
            if let Some(idx) = font_index("B微软雅黑") {
                Font::by_index(idx)
            } else if let Some(idx) = font_index("B黑体") {
                Font::by_index(idx)
            } else {
                panic!("未找到系统中文字体")
            }
        };

        let font_zh = {
            if let Some(idx) = font_index(" 微软雅黑") { 
                Font::by_index(idx)
            } else if let Some(idx) = font_index(" 黑体") {
                Font::by_index(idx)
            } else {
                panic!("未找到系统中文字体")
            }
        };

        let tgu_color = Color::from_rgb(113, 36, 107);
        let darkgrey = Color::from_rgb(80, 80, 80);

        Self {
            font_bold_zh,
            font_bold_en,
            font_zh,
            tgu_color,
            darkgrey,
        }
    }
}
