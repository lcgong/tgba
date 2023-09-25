use fltk::app;
use fltk::enums::{Align, Color};
use fltk::frame::Frame;
use fltk::group::{Flex, FlexType};
use fltk::image::{PngImage, SharedImage};
use fltk::prelude::{GroupExt, ImageExt, WidgetExt};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use super::style::AppStyle;

use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "resources/"]
struct EmbededResources;

const _N_FRAMES: usize = 8;
const _FRAME_INTERVAL: f64 = 1.0 / 15.0;
lazy_static! {
    static ref _FRAMES: Vec<SharedImage> = {
        let mut frames: Vec<SharedImage> = Default::default();
        for i in 1..=_N_FRAMES {
            let fname = format!("frame{}.png", i);
            let imgdata = EmbededResources::get(fname.as_str()).unwrap();
            let img = PngImage::from_data(&imgdata.data).unwrap();
            let mut img = SharedImage::from_image(img).unwrap();
            img.scale(22, 22, true, true);

            frames.push(img);
        }
        frames
    };

    static ref _ICON_EMPTY: SharedImage = {
        let imgdata = EmbededResources::get("frame0.png").unwrap();
        let img = PngImage::from_data(&imgdata.data).unwrap();
        let mut img = SharedImage::from_image(img).unwrap();
        img.scale(22, 22, true, true);
        img
    };

    static ref _ICON_SUCCESS: SharedImage = {
        let imgdata = EmbededResources::get("success.png").unwrap();
        let img = PngImage::from_data(&imgdata.data).unwrap();
        let mut img = SharedImage::from_image(img).unwrap();
        img.scale(32, 32, true, true);
        img
    };

    static ref _ICON_FAILURE: SharedImage = {
        let imgdata = EmbededResources::get("failure.png").unwrap();
        let img = PngImage::from_data(&imgdata.data).unwrap();
        let mut img = SharedImage::from_image(img).unwrap();
        img.scale(28, 28, true, true);
        img
    };
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TaskState {
    Todo = 0,
    Loading = 1,
    Done = 2,
    Error = 3,
}

pub struct Step {
    status_label: Frame,
    title_label: Frame,
    message_label: Frame,
    task_state: Arc<Mutex<TaskState>>,
    thread_handle: Mutex<Option<JoinHandle<()>>>,
}

const LOADING_ICON_SIZE: i32 = 32;
const STEP_PADDING: i32 = 10;

impl Step {
    pub fn add(style: &AppStyle, parent: &mut Flex) -> Self {
        let mut panel_flex = Flex::default();
        panel_flex.set_type(FlexType::Column);

        let mut title_row = Flex::default().row();

        let mut status_label = Frame::default();
        status_label.set_align(Align::Inside | Align::Center);
        status_label.set_image(Some(_ICON_EMPTY.clone()));
        title_row.fixed(&status_label, LOADING_ICON_SIZE);

        let mut title_label = Frame::default()
            .with_label("下载安装11111")
            .with_align(Align::Inside | Align::Left);
        title_label.set_label_font(style.step_title_font);
        title_label.set_label_size(style.step_title_fontsize);
        title_row.end();

        panel_flex.fixed(&title_row, style.step_title_fontsize);

        let mut message_row = Flex::default().row();
        let w = Frame::default();
        message_row.fixed(&w, LOADING_ICON_SIZE + 4);

        let mut message_label = Frame::default()
            .with_label("正在下载...")
            .with_align(Align::Inside | Align::Left);
        message_label.set_label_font(style.step_message_font);
        message_label.set_label_size(style.step_message_fontsize);
        message_row.end();

        panel_flex.fixed(&message_row, style.step_message_fontsize);
        panel_flex.end();

        parent.add(&panel_flex);
        parent.fixed(
            &panel_flex,
            style.step_title_fontsize + style.step_message_fontsize + STEP_PADDING,
        );

        Step {
            status_label,
            title_label,
            message_label,
            task_state: Arc::new(Mutex::new(TaskState::Todo)),
            thread_handle: Mutex::new(None),
        }
    }

    pub fn start(&self) {
        {
            let mut task_state = self.task_state.lock().unwrap();
            if *task_state == TaskState::Loading {
                return;
            }

            *task_state = TaskState::Loading;
        }

        let handle = std::thread::spawn({
            let mut next_frame_idx = 0;
            let mut status_label = self.status_label.clone();
            let task_state = self.task_state.clone();
            move || {
                println!("start frame");
                while *task_state.lock().unwrap() == TaskState::Loading {
                    let frame_idx = next_frame_idx;
                    next_frame_idx = (next_frame_idx + 1) % _N_FRAMES;

                    let frame = &_FRAMES[frame_idx];
                    status_label.set_image(Some(frame.clone()));
                    status_label.redraw();

                    // println!("frame {}", frame_idx);
                    app::sleep(_FRAME_INTERVAL);
                    app::awake();
                }

                println!("frame thread stopped!");
            }
        });

        *self.thread_handle.lock().unwrap() = Some(handle);
        app::awake();
    }

    fn stop(&self, state: TaskState) {
        let handle = { self.thread_handle.lock().unwrap().take() };
        if let Some(handle) = handle {
            *self.task_state.lock().unwrap() = state;
            if !handle.is_finished() {
                handle.join().unwrap();
            }
        } else {
            return;
        }

        println!("stop step");
    }

    pub fn set_title(&self, title: &str) {
        self.title_label.clone().set_label(title);
        app::awake();
    }

    pub fn set_message(&self, message: &str) {
        self.message_label.clone().set_label(message);
        app::awake();
    }

    pub fn success(&self) {
        self.stop(TaskState::Done);
        let mut status_label = self.status_label.clone();
        status_label.set_image(Some(_ICON_SUCCESS.clone()));
        status_label.redraw_label();

        let mut title_label = self.title_label.clone();
        title_label.set_label_color(Color::rgb_color(58, 179, 124));
        title_label.redraw_label();
        app::awake();
    }

    pub fn fail(&self) {
        self.stop(TaskState::Error);
        let mut status_label = self.status_label.clone();
        status_label.set_image(Some(_ICON_FAILURE.clone()));
        status_label.redraw_label();

        let mut title_label = self.title_label.clone();
        title_label.set_label_color(Color::rgb_color(174, 20, 49));
        title_label.redraw_label();
        app::awake();
        println!("render color");       
    }
}
