use fltk::{
    app::Sender,
    enums::{Align, Color},
    frame::Frame,
    group::{Flex, Group},
    misc::Progress,
    prelude::{GroupExt, WidgetBase, WidgetExt},
};
use std::path::PathBuf;

use super::super::pyenv::Installer;
use super::super::status::{DownloadingStats, StatusUpdate};
use super::super::{myapp::Message, style::AppStyle};

#[derive(Debug)]
pub enum Step2Message {
    Enter {
        target_dir: String,
    },
    Start,
    StartJob1,
    StartJob2,
    SuccessJob1,
    SuccessJob2,
    ErrorJob1(String),
    ErrorJob2(String),
    Job1Downloading {
        title: String,
        total_size: u64,
        percentage: f64,
        speed: f64,
    },
    Job1Message(String),
    Job2Message(String),
    Done,
}

pub struct Step2Tab {
    panel: Flex,
    sender: Sender<Message>,
    job1_spinner: LoadingSpinner,
    job1_message: Frame,
    job1_progress: Progress,
    job2_message: Frame,
    job2_spinner: LoadingSpinner,
}
use super::super::status::LoadingSpinner;

static GREY_COLOR: Color = Color::from_rgb(200, 200, 200);
static MESSAGE_COLOR: Color = Color::from_rgb(10, 10, 10);


impl Step2Tab {
    pub fn new(group: &mut Group, style: &AppStyle, sender: Sender<Message>) -> Self {
        let mut panel = Flex::default_fill().column();

        let job_title = ["下载安装Python", "创建Python虚拟环境"];

        panel.resize(group.x(), group.y(), group.w(), group.h());
        group.add(&panel);

        panel.set_margins(0, 20, 20, 20);

        Frame::default();

        let mut job1_spinner: LoadingSpinner;
        let mut job1_message: Frame;
        let mut job1_progress: Progress;
        let mut job2_message: Frame;
        let mut job2_spinner: LoadingSpinner;

        let mut row = Flex::default_fill().row();
        panel.fixed(&row, 32);
        {
            job1_spinner = LoadingSpinner::new(32);
            row.fixed(job1_spinner.widget(), 32);

            job1_message = Frame::default()
                .with_label(job_title[0])
                .with_align(Align::Inside | Align::Left);
            job1_message.set_label_color(GREY_COLOR);

            row.end();
        }

        let mut row = Flex::default_fill().row();
        panel.fixed(&row, 5);
        {
            let frame = Frame::default();
            row.fixed(&frame, 32);

            job1_progress = Progress::default();
            job1_progress.set_color(GREY_COLOR);
            job1_progress.set_frame(fltk::enums::FrameType::FlatBox);

            job1_progress.set_minimum(0.0);
            job1_progress.set_maximum(100.0);
            job1_progress.set_selection_color(style.tgu_color);

            row.end();
        }

        let frame = Frame::default();
        panel.fixed(&frame, 20);

        let mut row = Flex::default_fill().row();
        panel.fixed(&row, 32);
        {
            job2_spinner = LoadingSpinner::new(32);
            row.fixed(job2_spinner.widget(), 32);

            job2_message = Frame::default()
                .with_label(job_title[1])
                .with_align(Align::Inside | Align::Left);
            job2_message.set_label_color(GREY_COLOR);

            row.end();
        }

        Frame::default();

        panel.end();

        Step2Tab {
            panel,
            sender,
            job1_spinner,
            job1_message,
            job1_progress,
            job2_message,
            job2_spinner,
        }
    }

    pub fn widget(&self) -> &Flex {
        &self.panel
    }

    pub fn start(&mut self, target_dir: &str) {
        let installer = match Installer::new(PathBuf::from(target_dir)) {
            Ok(installer) => installer,
            Err(err) => {
                fltk::dialog::alert_default(&format!("初始化安装参数错误: {err}"));
                return;
            }
        };

        use tokio::runtime::Handle;
        let handle = Handle::current();
        // let progress = self.progress_bar.clone();
        // let message_label = self.message_label.clone();
        let installer = installer.clone();
        let sender = self.sender.clone();
        std::thread::spawn(move || {
            // 在新线程内运行异步代码
            handle.block_on(run(installer, sender));
        });
    }

    pub fn handle_message(&mut self, msg: Step2Message) {
        match msg {
            // Step2Message::Update => {
            //     //
            // }
            // Step2Message::Done => {
            //     //
            // }
            Step2Message::StartJob1 => {
                self.job1_spinner.start();
                self.job1_message.set_label_color(MESSAGE_COLOR);
                self.job1_message.redraw();
            }
            Step2Message::SuccessJob1 => {
                self.job1_spinner.success();
            }
            Step2Message::ErrorJob1(err) => {
                self.job1_spinner.error();
                fltk::dialog::alert_default(&err);
            }
            Step2Message::StartJob2 => {
                self.job2_spinner.start();
                self.job2_message.set_label_color(MESSAGE_COLOR);
                self.job2_message.redraw();
            }
            Step2Message::SuccessJob2 => {
                self.job2_spinner.success();
            }
            Step2Message::ErrorJob2(err) => {
                self.job2_spinner.error();
                fltk::dialog::alert_default(&err);
            }
            Step2Message::Job1Message(message) => {
                self.job1_message.set_label(&message);
            }
            Step2Message::Job1Downloading {
                title,
                total_size,
                percentage,
                speed,
            } => {
                job1_downloading(
                    &mut self.job1_message,
                    &mut self.job1_progress,
                    &title,
                    total_size,
                    percentage,
                    speed,
                );
            }
            Step2Message::Job2Message(message) => {
                self.job1_message.set_label(&message);
            }
            msg @ _ => {
                println!("unimplemented {:?}", msg);
            }
        }
    }
}

fn job1_downloading(
    label: &mut Frame,
    progress: &mut Progress,
    title: &str,
    total_size: u64,
    percentage: f64,
    speed: f64,
) {
    let total_size = format_scale(total_size as f64, 1);
    let speed = format_scale(speed as f64, 2);

    let msg = format!("{title}, 大小: {total_size} - 速度: {speed}/s");
    label.set_label(&msg);
    progress.set_value(percentage);
}

fn job2_update(label: &mut Frame, message: &str) {
    label.set_label(&message);
}

pub struct StatusUpdater {
    job_idx: u32,
    sender: Sender<Message>,
}

impl StatusUpdater {
    pub fn new(sender: Sender<Message>) -> Self {
        StatusUpdater { job_idx: 0, sender }
    }
}

impl StatusUpdater {
    pub fn start_job(&mut self, job_idx: u32) {
        self.job_idx = job_idx;

        if job_idx == 0 {
            self.sender.send(Message::Step2(Step2Message::StartJob1));
        } else if job_idx == 1 {
            self.sender.send(Message::Step2(Step2Message::StartJob2));
        } else {
            unimplemented!();
        }
    }

    pub fn success_job(&mut self, job_idx: u32) {
        self.job_idx = job_idx;

        if job_idx == 0 {
            self.sender.send(Message::Step2(Step2Message::SuccessJob1));
        } else if job_idx == 1 {
            self.sender.send(Message::Step2(Step2Message::SuccessJob2));
        } else {
            unimplemented!();
        }
    }

    pub fn error_job(&mut self, job_idx: u32, err: String) {
        self.job_idx = job_idx;

        if job_idx == 0 {
            self.sender
                .send(Message::Step2(Step2Message::ErrorJob1(err)));
        } else if job_idx == 1 {
            self.sender
                .send(Message::Step2(Step2Message::ErrorJob2(err)));
        } else {
            unimplemented!();
        }
    }
}

impl StatusUpdate for StatusUpdater {
    fn alert(&self, err: &str) {
        let err = err.to_string();
        fltk::app::awake_callback(move || {
            fltk::dialog::alert_default(&err);
        });
    }

    fn message(&self, msg: &str) {
        if self.job_idx == 0 {
            self.sender
                .send(Message::Step2(Step2Message::Job1Message(msg.to_string())));
        } else if self.job_idx == 1 {
            self.sender
                .send(Message::Step2(Step2Message::Job2Message(msg.to_string())));
        } else {
            unimplemented!();
        }
    }

    fn update_progress(&self, value: f64) {
        unimplemented!();
    }

    fn update_downloading(&self, status: &DownloadingStats) {
        if self.job_idx == 0 {
            let sender = self.sender.clone();
            let title = status.title().to_string();
            let total_size = status.total_size();
            let speed = status.speed();
            let percentage = status.percentage();

            fltk::app::awake_callback(move || {
                sender.send(Message::Step2(Step2Message::Job1Downloading {
                    title: title.clone(),
                    total_size,
                    percentage,
                    speed,
                }));
            });
        }
    }
}

fn format_scale(size: f64, precision: usize) -> String {
    let scale_kb = 2u64.pow(10) as f64;
    if size < scale_kb {
        return format!("{size:.0}B");
    }

    let scale_mb = 2u64.pow(20) as f64;
    if size < scale_mb {
        let size = (size as f64) / (scale_kb as f64);
        return format!("{size:.*}KiB", precision);
    }

    let scale_gb = 2u64.pow(30) as f64;
    if size < scale_gb {
        let size = (size as f64) / (scale_mb as f64);
        return format!("{size:.*}MiB", precision);
    }

    let size = (size as f64) / (scale_gb as f64);
    return format!("{size:.*}GiB", precision);
}

pub async fn run(mut installer: Installer, sender: Sender<Message>) {
    println!("started - step1");

    use super::super::pyenv::venv::{ensure_python_dist, ensure_venv, set_platform_info};

    let mut updater = StatusUpdater::new(sender);

    updater.start_job(0);
    // if let Err(err) = ensure_python_dist(&mut installer, &updater).await {
    //     updater.alert(&format!("下载安装CPython中发生错误: {err}"));
    // };
                use tokio::time::{sleep, Duration};
                sleep(Duration::from_millis(5000)).await;

    updater.success_job(0);

    updater.start_job(1);

    sleep(Duration::from_millis(5000)).await;

    // if let Err(err) = ensure_venv(&mut installer, &updater).await {
    //     updater.error_job(1, err.to_string());
    //     // updater.alert(&format!("创建Python虚拟环境发生错误: {err}"));
    // };

    // if let Err(err) = set_platform_info(&mut installer) {
    //     updater.alert(&format!("获取Python本地平台信息发生错误: {err}"));
    // };
    updater.success_job(1);

    // ensure_python_dist(installer, status_update).await?;

    // ensure_venv(installer, status_update).await?;

    // set_platform_info(installer)?;
}

// pub async fn run2(progress: Progress) {
//     // progress.set_callback(cb);
//     //
//     println!("started - step1");
//     for i in 1..101 {
//         use std::thread::sleep;
//         use std::time::Duration;
//         sleep(Duration::from_millis(100));
//         let msg = format!("Hello world-{:3}", i);
//         println!("msg: {msg}");

//         fltk::app::awake_callback({
//             let mut progress = progress.clone();
//             move || {
//                 progress.set_value(i as f64);
//             }
//         });
//     }
// }
