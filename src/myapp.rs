use fltk::{
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
        step4::{Step4Message, Step4Tab},
        step5::{Step5Message, Step5Tab},
        step6::{Step6Message, Step6Tab},
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
    Step4(Step4Message),
    Step5(Step5Message),
    Step6(Step6Message),
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

        let mut objs: Vec<Box<dyn Any>> = Vec::new();
        objs.push(Box::new(Step1Tab::new(&mut tabs, &style, s.clone())));
        objs.push(Box::new(Step2Tab::new(&mut tabs, &style, s.clone())));
        objs.push(Box::new(Step3Tab::new(&mut tabs, &style, s.clone())));
        objs.push(Box::new(Step4Tab::new(&mut tabs, &style, s.clone())));
        objs.push(Box::new(Step5Tab::new(&mut tabs, &style, s.clone())));
        objs.push(Box::new(Step6Tab::new(&mut tabs, &style, s.clone())));

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
                Step1(msg @ Step1Message::Enter) => {
                    self.set_step(0);
                    let d = self.get_step_mut::<Step1Tab>();
                    d.handle_message(msg);
                }
                Step1(Step1Message::Done) => {
                    println!("step1: done");
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
                    d.handle_message(msg);
                }
                Step2(Step2Message::Done) => {
                    println!("step2: done");
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
                    s.send(Step4(Step4Message::Enter));
                }
                Step3(msg) => {
                    let d = self.get_step_mut::<Step3Tab>();
                    d.handle_message(msg);
                }

                //
                Step4(msg @ Step4Message::Enter) => {
                    self.set_step(3);
                    let d = self.get_step_mut::<Step4Tab>();
                    d.c();
                    d.handle_message(msg);
                }
                Step4(Step4Message::Done) => {
                    println!("step4: done");
                    s.send(Step5(Step5Message::Enter));
                }
                Step4(msg) => {
                    let d = self.get_step_mut::<Step4Tab>();
                    d.handle_message(msg);
                }
                //
                Step5(msg @ Step5Message::Enter) => {
                    self.set_step(4);
                    let d = self.get_step_mut::<Step5Tab>();
                    d.c();
                    d.handle_message(msg);
                }
                Step5(Step5Message::Done) => {
                    println!("step5: done");
                    s.send(Step6(Step6Message::Enter));
                }
                Step5(msg) => {
                    let d = self.get_step_mut::<Step5Tab>();
                    d.handle_message(msg);
                }
                //
                Step6(msg @ Step6Message::Enter) => {
                    self.set_step(5);
                    let d = self.get_step_mut::<Step6Tab>();
                    d.c();
                    d.handle_message(msg);
                }
                Step6(Step6Message::Done) => {
                    println!("step6: done");
                    // s.send(Step6(Step6Message::Enter));
                }
                Step6(msg) => {
                    let d = self.get_step_mut::<Step6Tab>();
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
