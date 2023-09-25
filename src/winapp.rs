// #![windows_subsystem = "windows"]
// 在debug模式下终端显示print，发行版不显示终端窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use fltk::app;
use fltk::enums::Event;
use fltk::frame::Frame;
use fltk::group::{Flex, FlexType};
use fltk::prelude::*;
use fltk::window::Window;

use super::heading::build_heading;
use super::step::Step;
use super::style::AppStyle;

pub struct MyEvent;

pub struct StepGroup {
    pub steps: Vec<Step>,
    step_idx: Mutex<usize>,
}

impl StepGroup {
    pub fn new() -> Self {
        Self {
            steps: Default::default(),
            step_idx: Mutex::new(0),
        }
    }

    pub fn push(&mut self, step: Step) {
        self.steps.push(step);
    }

    pub fn start(&self) {
        match self.step_idx.lock() {
            Ok(step_idx) => {
                self.steps[*step_idx].start();
            }
            Err(_) => panic!("cannot get step_idx's lock"),
        }
    }

    pub fn success(&self) {
        match self.step_idx.lock() {
            Ok(step_idx) => {
                self.steps[*step_idx].success();
            }
            Err(_) => panic!("cannot get step_idx's lock"),
        }
    }

    pub fn fail(&self) {
        match self.step_idx.lock() {
            Ok(step_idx) => {
                self.steps[*step_idx].fail();
            }
            Err(_) => panic!("cannot get step_idx's lock"),
        }
    }

    pub fn next_step(&self) {
        match self.step_idx.lock() {
            Ok(mut step_idx) => {
                *step_idx += 1;
            }
            Err(_) => panic!("cannot get step_idx's lock"),
        }
    }

    pub fn set_title(&self, title: &str) {
        match self.step_idx.lock() {
            Ok(step_idx) => {
                self.steps[*step_idx].set_title(title);
            }
            Err(_) => panic!("cannot get step_idx's lock"),
        }
    }

    pub fn set_message(&self, message: &str) {
        match self.step_idx.lock() {
            Ok(step_idx) => {
                self.steps[*step_idx].set_message(message);
            }
            Err(_) => panic!("cannot get step_idx's lock"),
        }
    }
}

pub fn main_app() {
    let fltk_app = app::App::default();
    let style = AppStyle::default(&fltk_app);

    app::background(255, 255, 255);
    app::set_visible_focus(false);

    let mut window = Window::default().with_size(800, 600).center_screen();
    window.set_label("TGBA商务数据分析实验平台自助安装");
    // window.set_border(false);

    let mut win_flex = Flex::default().size_of_parent();
    win_flex.set_margin(25);

    let mut body_flex = Flex::default().size_of_parent();
    body_flex.set_type(FlexType::Column);

    build_heading(&fltk_app, &mut body_flex, &style);

    let mut _padding = Frame::default();
    body_flex.set_size(&mut _padding, 12);

    let mut steps_flex = Flex::default().size_of_parent();
    steps_flex.set_type(FlexType::Column);
    steps_flex.end();

    let _padding = Frame::default();

    body_flex.end();
    win_flex.end();
    window.end();
    window.show();

    window.set_callback(move |_| {
        if app::event() == Event::Close {
            println!("exit app!");
            fltk_app.quit();
        }
    });
    let mut g = StepGroup::new();
    g.push(Step::add(&style, &mut steps_flex.clone()));
    g.push(Step::add(&style, &mut steps_flex.clone()));
    g.push(Step::add(&style, &mut steps_flex.clone()));
    g.push(Step::add(&style, &mut steps_flex.clone()));

    let g = Arc::new(g);

    std::thread::spawn({
        let g = g.clone();
        move || {
            g.start();
            app::sleep(3.0);
            g.success();
            g.next_step();
            g.start();
            app::sleep(1.0);
            g.set_message("更新设置99999");
            g.fail();
        }
    });

    fltk_app.run().unwrap();
}
