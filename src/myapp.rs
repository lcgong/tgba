use fltk::frame::Frame;
use fltk::group::Flex;
use fltk::prelude::GroupExt;
use fltk::prelude::WindowExt;
use fltk::prelude::{WidgetBase, WidgetExt};
use fltk::window::DoubleWindow;
// use fltk::prelude::{ImageExt};
use fltk::enums::Color;
use fltk::image::IcoImage;
// use fltk::image::SvgImage;
use fltk::app::font_index;
use fltk::enums::Font;

pub struct AppStyle {
    pub font_bold_zh: Font,
    pub font_bold_en: Font,
    pub font_zh: Font,
    pub tgu_color: Color,
    pub darkgrey: Color,
}

impl Default for AppStyle {
    fn default() -> Self {
        let font_bold_en = { Font::by_index(font_index("BArial").unwrap()) };
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
            if let Some(idx) = font_index("B微软雅黑") {
                Font::by_index(idx)
            } else if let Some(idx) = font_index("B黑体") {
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

pub struct MyApp {
    app: fltk::app::App,
    r: fltk::app::Receiver<Message>,
    s: fltk::app::Sender<Message>,
    panel: Flex,
}

pub enum Message {
    Step1,
    Step2,
}


fn app_title(parent: &mut Flex, style: &AppStyle) {
    use fltk::enums::Align;

    let panel = Flex::default().column();
    parent.fixed(&panel, 42);

    let mut title_zh = Frame::default()
        .with_label("天工业务数据分析(TGBA)实验环境自助安装")
        .with_align(Align::Inside | Align::Left);
    title_zh.set_label_font(style.font_bold_zh);
    title_zh.set_label_size(22);
    title_zh.set_label_color(style.tgu_color);

    let mut title_en = Frame::default()
        .with_label("TianGong Business Analytics (TGBA) Installer")
        .with_align(Align::Inside | Align::Left);
    title_en.set_label_font(style.font_bold_en);
    title_en.set_label_size(18);
    title_en.set_label_color(style.darkgrey);

    panel.end();
}

fn app_footer(parent: &mut Flex, style: &AppStyle) {
    use fltk::enums::Align;

    let mut footer = Frame::default()
        .with_label("天津工业大学经济与管理学院 © 2023")
        .with_align(Align::Inside | Align::Right);

    footer.set_label_font(style.font_zh);
    footer.set_label_size(12);
    footer.set_label_color(style.darkgrey);

    parent.fixed(&footer, 18);
}



impl MyApp {
    pub fn new() -> Self {
        let app = fltk::app::App::default().with_scheme(fltk::app::Scheme::Gtk);
        fltk::app::background(255, 255, 255);
        app.load_system_fonts();

        let style = AppStyle::default();

        // let fonts = fltk::app::fonts();
        // for f in fonts {
        //     println!("font: {}", f);
        // }
        let (s, r) = fltk::app::channel::<Message>();

        let icon = IcoImage::load(&"resources/tgba-jupyterlab-48x48.ico").unwrap();

        let mut main_win = DoubleWindow::default()
            .with_size(700, 400)
            .center_screen()
            .with_label("TGBA安装程序");
        main_win.set_icon(Some(icon));
        // main_win.make_resizable(true);

        let mut main_flex = Flex::default_fill().column();
        main_flex.set_margins(10, 10, 10, 10);

        app_title(&mut main_flex, &style);

        let panel = Flex::default().row();
        panel.end();

        app_footer(&mut main_flex, &style);

        main_win.end();
        main_win.show();

        let s1 = s.clone();
        tokio::spawn(async move {
            use std::time::Duration;
            tokio::time::sleep(Duration::from_millis(2000)).await;
            s1.send(Message::Step1);
        });

        MyApp { app, r, s, panel }
    }

    pub fn run(&mut self) {
        while self.app.wait() {
            let Some(msg) = self.r.recv() else {
                continue;
            };

            let s = self.s.clone();

            use Message::*;
            match msg {
                Step1 => {
                    self.panel.clear();
                    let btn = fltk::button::Button::default().with_label("测试1");
                    self.panel.add(&btn);

                    tokio::spawn(async move {
                        use std::time::Duration;
                        tokio::time::sleep(Duration::from_millis(2000)).await;
                        s.send(Step2);
                    });
                }
                Step2 => {
                    self.panel.clear();
                    let btn = fltk::button::Button::default().with_label("测试2");
                    self.panel.add(&btn);

                    tokio::spawn(async move {
                        use std::time::Duration;
                        tokio::time::sleep(Duration::from_millis(2000)).await;
                        s.send(Step1);
                    });
                }
            }
        }
    }

    pub fn choose_target_dir_panel(&mut self) {
    
    }
}
