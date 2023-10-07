use fltk::{
    button::Button,
    frame::Frame,
    group::{Flex, Group},
    image::IcoImage,
    prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt},
    window::DoubleWindow,
};
use std::any::Any;

use super::{
    navbar::PhaseNavBar,
    steps::{
        step1::{Step1Message, Step1Tab},
        step2::{Step2Message, Step2Tab},
        step3::{Step3Message, Step3Tab},
    },
    style::AppStyle,
};
//

pub struct MyApp {
    app: fltk::app::App,
    r: fltk::app::Receiver<Message>,
    s: fltk::app::Sender<Message>,
    // container: Flex,
    navbar: PhaseNavBar,
    // main_panel: Option<InstallerPanel>,
    step_idx: usize,
    tabs: Group,
    objs: Vec<Box<dyn Any>>,
}

#[derive(Debug)]
pub enum Message {
    Step1(Step1Message),
    Step2(Step2Message),
    Step3(Step3Message),
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
        .with_label("TianGong Business Analytics (TGBA) Lab Installer")
        .with_align(Align::Inside | Align::Left);
    title_en.set_label_font(style.font_bold_en);
    title_en.set_label_size(16);
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

        let (s, r) = fltk::app::channel::<Message>();

        let icon = IcoImage::load(&"resources/tgba-jupyterlab-48x48.ico").unwrap();

        let mut main_win = DoubleWindow::default()
            .with_size(700, 300)
            .center_screen()
            .with_label("TGBA安装程序");
        main_win.set_icon(Some(icon));
        // main_win.make_resizable(true);

        let mut main_flex = Flex::default_fill().column();
        main_flex.set_margins(10, 10, 10, 10);

        app_title(&mut main_flex, &style);

        let navbar = PhaseNavBar::new(&style);
        main_flex.fixed(navbar.navbar_row(), 40);

        let mut tabs = Group::default_fill();
        tabs.end();

        app_footer(&mut main_flex, &style);

        main_win.end();
        main_win.show();

        s.send(Message::Step1(Step1Message::Enter));

        let step1 = Step1Tab::new(&mut tabs, &style, s.clone(), r.clone());
        let step2 = Step2Tab::new(&mut tabs, &style, s.clone(), r.clone());
        let step3 = Step3Tab::new(&mut tabs, &style, s.clone(), r.clone());

        let mut flex = Flex::default().row();
        flex.resize(tabs.x(), tabs.y(), tabs.w(), tabs.h());
        let btn = Button::default().with_label("B1");
        flex.fixed(&btn, 40);
        let btn = Button::default().with_label("B2");
        flex.fixed(&btn, 40);
        tabs.add(&flex);

        let mut flex = Flex::default().column();
        flex.resize(tabs.x(), tabs.y(), tabs.w(), tabs.h());
        let btn = Button::default().with_label("C1");
        flex.fixed(&btn, 40);
        let btn = Button::default().with_label("C2");
        flex.fixed(&btn, 40);
        tabs.add(&flex);

        let mut objs: Vec<Box<dyn Any>> = Vec::new();
        objs.push(Box::new(step1));
        objs.push(Box::new(step2));
        objs.push(Box::new(step3));

        MyApp {
            app,
            r,
            s,
            step_idx: 0,
            tabs,
            navbar,
            objs, // main_panel: None,
        }
    }

    fn set_step(&mut self, step_idx: usize) {
        self.step_idx = step_idx;
        self.navbar.set_activate(step_idx as i32);

        let step_idx = step_idx as i32;
        for idx in 0..self.tabs.children() {
            if idx != step_idx {
                self.tabs.child(idx).unwrap().hide();
            } else {
                self.tabs.child(idx).unwrap().show();
            }
        }
    }

    #[inline]
    fn get_step_mut<T: Any>(&mut self) -> &mut T {
        self.objs[self.step_idx].downcast_mut::<T>().unwrap()
    }

    pub fn run(&mut self) {
        while self.app.wait() {
            let Some(msg) = self.r.recv() else {
                continue;
            };

            let s = self.s.clone();

            use Message::*;
            match msg {
                Step1(Step1Message::Enter) => {
                    self.set_step(0);
                    // let d = self.get_step_mut::<Step1Tab>();
                }
                Step1(Step1Message::Done) => {
                    s.send(Step2(Step2Message::Enter));
                }
                Step1(msg) => {
                    let d = self.get_step_mut::<Step1Tab>();
                    d.handle_message(msg);
                }
                //
                Step2(msg @ Step2Message::Enter) => {
                    self.set_step(1);
                    let d = self.get_step_mut::<Step2Tab>();
                    d.b();
                    d.handle_message(msg);
                }
                Step2(Step2Message::Done) => {
                    s.send(Step3(Step3Message::Enter));
                }
                Step2(msg) => {
                    let d = self.get_step_mut::<Step2Tab>();
                    d.handle_message(msg);
                }
                //
                Step3(msg @ Step3Message::Enter) => {
                    self.set_step(2);
                    let d = self.get_step_mut::<Step3Tab>();
                    d.c();
                    d.handle_message(msg);
                }
                Step3(Step3Message::Done) => {
                    println!("step3: done");
                    s.send(Step1(Step1Message::Enter));
                }
                Step3(msg) => {
                    let d = self.get_step_mut::<Step3Tab>();
                    d.handle_message(msg);
                }
            }
        }
    }

    pub fn choose_target_dir_panel(&mut self) {}
}

impl Default for MyApp {
    fn default() -> Self {
        Self::new()
    }
}
