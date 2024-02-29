use fltk::{
    app::Sender,
    enums::{Align, Color},
    frame::Frame,
    group::{Flex, Group},
    misc::Progress,
    prelude::{GroupExt, WidgetBase, WidgetExt},
};
use std::path::PathBuf;

use super::super::{
    myapp::Message,
    pyenv::{ensure_python_dist, ensure_venv, Installer},
    status::{DownloadingStats, LoadingSpinner, StatusUpdate},
    steps::utils::format_scale,
    style::AppStyle,
};

#[derive(Debug)]
pub enum Step2Message {
    Enter {
        target_dir: String,
    },
    StartJob(usize),
    SuccessJob(usize),
    ErrorJob(usize, String),   // (job_idx, errmsg)
    JobMessage(usize, String), // (job_idx, message)
    Job1Downloading {
        title: String,
        total_size: u64,
        percentage: f64,
        speed: f64,
    },
    Done(Installer),
}

pub struct Step2Tab {
    panel: Flex,
    sender: Sender<Message>,
    job_messages: Vec<Frame>,
    job_spinners: Vec<LoadingSpinner>,
    job1_progress: Progress,
    installer: Option<Installer>,
}

static GREY_COLOR: Color = Color::from_rgb(200, 200, 200);
static MESSAGE_COLOR: Color = Color::from_rgb(10, 10, 10);

impl Step2Tab {
    pub fn new(group: &mut Group, style: &AppStyle, sender: Sender<Message>) -> Self {
        let mut panel = Flex::default_fill().column();

        let job_title = ["下载安装Python", "创建Python虚拟环境"];

        panel.resize(group.x(), group.y(), group.w(), group.h());
        group.add(&panel);

        panel.set_margins(40, 20, 40, 20);

        Frame::default();

        let mut job_spinners: Vec<LoadingSpinner> = Vec::new();
        let mut job_messages: Vec<Frame> = Vec::new();
        let mut download_progress: Progress;

        // ---------------- Job0 ------------------------------------------
        let mut job_flex = Flex::default_fill().row();
        panel.fixed(&job_flex, 32);
        {
            let job1_spinner = LoadingSpinner::new(36);
            job_flex.fixed(job1_spinner.widget(), 36);
            job_spinners.push(job1_spinner);

            let mut flex = Flex::default_fill().column();
            flex.set_margins(0, 0, 0, 0);
            flex.set_spacing(0);
            {
                let mut job_message = Frame::default()
                    .with_label(job_title[0])
                    .with_align(Align::Inside | Align::Left);
                job_message.set_label_size(16);
                job_message.set_label_color(GREY_COLOR);
                job_messages.push(job_message);

                download_progress = Progress::default();
                flex.fixed(&download_progress, 4);

                download_progress.set_color(GREY_COLOR);
                download_progress.set_frame(fltk::enums::FrameType::FlatBox);

                download_progress.set_minimum(0.0);
                download_progress.set_maximum(100.0);
                download_progress.set_selection_color(style.tgu_color);

                flex.end();
            }
            job_flex.end();
        }

        panel.fixed(&Frame::default(), 10); // 间隔空行

        //--------------- Job 1 ----------------------------------------------
        let mut job_flex = Flex::default_fill().row();
        panel.fixed(&job_flex, 32);
        {
            let spinner = LoadingSpinner::new(36);
            job_flex.fixed(spinner.widget(), 36);
            job_spinners.push(spinner);

            let mut flex = Flex::default_fill().column();
            flex.set_margins(0, 0, 0, 0);
            flex.set_spacing(0);
            {
                let mut job_message = Frame::default()
                    .with_label(job_title[1])
                    .with_align(Align::Inside | Align::Left);
                job_message.set_label_size(16);
                job_message.set_label_color(GREY_COLOR);
                job_messages.push(job_message);

                flex.fixed(&Frame::default(), 4);

                flex.end();
            }
            job_flex.end();
        }

        Frame::default();

        panel.end();

        Step2Tab {
            panel,
            sender,
            job_spinners,
            job_messages,
            job1_progress: download_progress,
            installer: None,
        }
    }

    pub fn widget(&self) -> &Flex {
        &self.panel
    }

    pub fn start(&mut self, target_dir: &str) {
        let mut collector = StatusCollector::new(self.sender.clone(), 0);

        let installer = match Installer::new(PathBuf::from(target_dir)) {
            Ok(installer) => installer,
            Err(err) => {
                collector.job_error(format!("初始化安装参数错误: {err}"));
                return;
            }
        };

        self.installer = Some(installer.clone());

        collector.job_start();
    }

    fn on_job_start(&mut self, job_idx: usize) {
        println!("on_job_start: {job_idx}");
        let mut collector = StatusCollector::new(self.sender.clone(), job_idx);

        if job_idx >= 2 {
            collector.done(self.installer.clone().unwrap());
            return;
        }

        self.job_spinners[job_idx].start();
        let message_label = &mut self.job_messages[job_idx];
        message_label.set_label_color(MESSAGE_COLOR);
        message_label.redraw();

        use tokio::runtime::Handle;

        let installer = self.installer.clone().unwrap();
        let handle = Handle::current();

        if job_idx == 0 {
            std::thread::spawn(move || {
                // 在新线程内运行异步代码
                handle.block_on(download_worker(installer, collector));
            });
        } else if job_idx == 1 {
            std::thread::spawn(move || {
                // 在新线程内运行异步代码
                handle.block_on(venv_worker(installer, collector));
            });
        }
    }

    fn on_job_success(&mut self, job_idx: usize) {
        self.job_spinners[job_idx].success();
        let message_label = &mut self.job_messages[job_idx];
        message_label.set_label_color(MESSAGE_COLOR);
        message_label.redraw();

        self.sender.send(Message::Step2(Step2Message::StartJob(job_idx + 1)));
    }

    fn on_job_error(&mut self, job_idx: usize, err: String) {
        self.job_spinners[job_idx].error();
        fltk::dialog::alert_default(&err);
    }

    pub fn handle_message(&mut self, msg: Step2Message) {
        match msg {
            Step2Message::StartJob(job_idx) => self.on_job_start(job_idx),
            Step2Message::SuccessJob(job_idx) => self.on_job_success(job_idx),
            Step2Message::ErrorJob(job_idx, err) => self.on_job_error(job_idx, err),
            Step2Message::JobMessage(job_idx, message) => {
                self.job_messages[job_idx].set_label(&message);
            }
            Step2Message::Job1Downloading {
                title,
                total_size,
                percentage,
                speed,
            } => {
                let total_size = format_scale(total_size as f64, 1);
                let speed = format_scale(speed as f64, 2);

                let msg = format!("{title}, {total_size} \t {speed}/s");
                self.job_messages[0].set_label(&msg);
                self.job1_progress.set_value(percentage);
            }
            msg @ _ => {
                log::error!("unimplemented {:?}", msg);
            }
        }
    }
}

#[derive(Debug)]
pub struct StatusCollector {
    job_idx: usize,
    sender: Sender<Message>,
}

impl StatusCollector {
    pub fn new(sender: Sender<Message>, job_idx: usize) -> Self {
        StatusCollector { job_idx, sender }
    }

    pub fn job_start(&mut self) {
        self.send(Step2Message::StartJob(self.job_idx));
    }

    pub fn job_success(&mut self) {
        self.send(Step2Message::SuccessJob(self.job_idx));
    }

    pub fn job_error(&self, err: String) {
        self.send(Step2Message::ErrorJob(self.job_idx, err));
    }

    pub fn done(&mut self, installer: Installer) {
        self.send(Step2Message::Done(installer));
    }

    fn send(&self, msg: Step2Message) {
        self.sender.send(Message::Step2(msg));
    }
}

impl StatusUpdate for StatusCollector {
    fn message(&self, msg: &str) {
        log::info!("{msg}");
        self.send(Step2Message::JobMessage(self.job_idx, msg.to_string()));
    }

    fn update_downloading(&self, status: &DownloadingStats) {
        if self.job_idx != 0 {
            return;
        }

        let sender = self.sender.clone();
        let title = status.title().to_string();
        let total_size = status.total_size();
        let speed = status.speed();
        let percentage = status.percentage();

        // fltk::app::awake_callback(move || {
        sender.send(Message::Step2(Step2Message::Job1Downloading {
            title: title.clone(),
            total_size,
            percentage,
            speed,
        }));
        // });
    }
}

pub async fn download_worker(mut installer: Installer, mut collecter: StatusCollector) {
    println!("download_worker start");
    if let Err(err) = ensure_python_dist(&mut installer, &collecter).await {
        collecter.job_error(format!("下载安装CPython中发生错误: {err}"));
        return;
    };
    println!("download_worker done");

    collecter.job_success();
    // collecter.next_job();
}

pub async fn venv_worker(mut installer: Installer, mut collecter: StatusCollector) {
    println!("venv_worker start");
    if let Err(err) = ensure_venv(&mut installer, &collecter).await {
        collecter.job_error(format!("创建Python虚拟环境发生错误: {err}"));
        return;
    };
    println!("venv_worker done");

    collecter.job_success();
    // collecter.next_job();
}

// pub async fn step_run(mut installer: Installer, mut collecter: StatusCollector) {
//     if collecter.job_idx == 0 {
//         collecter.job_start();
//         println!("start111");
//         if let Err(err) = ensure_python_dist(&mut installer, &collecter).await {
//             println!("errr");
//             collecter.job_error(format!("下载安装CPython中发生错误: {err}"));
//             return;
//         };

//         println!("job done");

//         collecter.job_success(installer);
//         collecter.next_job();
//     } else if collecter.job_idx == 1 {

//     }
// }

// tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
