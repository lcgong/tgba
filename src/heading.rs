use super::style::AppStyle;
use fltk::app;
use fltk::button::Button;
use fltk::enums::{Align, FrameType};
use fltk::frame::Frame;
use fltk::group::{Flex, FlexType};
use fltk::prelude::{GroupExt, WidgetExt};

pub fn build_heading(fltk_app: &app::App, parent: &mut Flex, style: &AppStyle) {
    let mut heading_flex = Flex::default();
    heading_flex.set_type(FlexType::Row);
    {
        let mut content = Flex::default();
        content.set_type(FlexType::Column);

        let mut label_title = Frame::default()
            .with_label("商务数据分析实验平台 自助安装")
            .with_align(Align::Inside | Align::Left);
        label_title.set_label_font(style.prog_title_font);
        label_title.set_label_size(26);
        label_title.set_label_color(style.prog_title_color);
        content.set_size(&mut label_title, 26);

        let mut label_title_en = Frame::default()
            .with_label("TianGong Business Analytics (TGBA)")
            .with_align(Align::Inside | Align::Left);
        label_title_en.set_label_font(style.prog_title_en_font);
        label_title_en.set_label_size(22);
        label_title_en.set_label_color(style.prog_title_en_color);
        content.set_size(&mut label_title_en, 22);

        let mut label_copyright = Frame::default()
            .with_label("天津工业大学经济与管理学院 © 2023")
            .with_align(Align::Inside | Align::Left);
        label_copyright.set_label_font(style.copyright_font);
        label_copyright.set_label_size(18);
        label_copyright.set_label_color(style.copyright_color);
        content.set_size(&mut label_copyright, 18);

        content.end();
    }
    {
        let mut heading_btn_flex = Flex::default().column();
        let mut exit_btn = Button::default().with_label("中断安装");
        exit_btn.set_color(style.exit_btn_color);
        exit_btn.set_selection_color(style.exit_btn_sel_color);
        exit_btn.set_label_color(style.exit_btn_label_color);
        exit_btn.set_frame(FrameType::FlatBox);

        exit_btn.set_callback({
            let fltk_app = fltk_app.clone();
            move |_| {
                fltk_app.quit();
            }
        });
        heading_btn_flex.fixed(&mut exit_btn, 40);
        heading_btn_flex.end();
        heading_flex.fixed(&mut heading_btn_flex, 100);
    }
    heading_flex.end();

    parent.fixed(&heading_flex, 80);
}
