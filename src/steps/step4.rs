use super::super::{
    myapp::Message,
    pyenv::Installer,
    status::LoadingSpinner,
    status::{DownloadingStats, StatusUpdate},
    style::AppStyle,
};
use fltk::{
    app::Sender,
    enums::{Align, Color},
    frame::Frame,
    group::{Flex, Group},
    prelude::{GroupExt, WidgetBase, WidgetExt},
};

#[derive(Debug)]
pub enum Step4Message {
    Enter(Installer),
    JobStart(usize),           // (job_idx)
    JobSuccess(usize),         // (job_idx)
    JobMessage(usize, String), // (job_idx, message)
    JobError(usize, String),   // (job_idx, errmsg)
    Done(Installer),
}

pub struct Step4Tab {
    panel: Flex,
    sender: Sender<Message>,
    job_messages: Vec<Frame>,
    job_spinners: Vec<LoadingSpinner>,
    installer: Option<Installer>,
}

static GREY_COLOR: Color = Color::from_rgb(200, 200, 200);
static MESSAGE_COLOR: Color = Color::from_rgb(10, 10, 10);

fn render_job_status(
    title: &str,
    panel: &mut Flex,
    job_spinners: &mut Vec<LoadingSpinner>,
    job_messages: &mut Vec<Frame>,
) {
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
            let mut message = Frame::default()
                .with_label(title)
                .with_align(Align::Inside | Align::Left);
            message.set_label_size(16);
            message.set_label_color(GREY_COLOR);
            job_messages.push(message);

            flex.fixed(&Frame::default(), 4);

            flex.end();
        }
        job_flex.end();
    }
}

impl Step4Tab {
    pub fn new(group: &mut Group, _style: &AppStyle, sender: Sender<Message>) -> Self {
        let mut panel = Flex::default_fill().column();

        panel.resize(group.x(), group.y(), group.w(), group.h());
        group.add(&panel);

        panel.set_margins(40, 20, 40, 20);

        Frame::default();

        let mut job_spinners: Vec<LoadingSpinner> = Vec::new();
        let mut job_messages: Vec<Frame> = Vec::new();

        render_job_status(
            "安装Python本地程序包",
            &mut panel,
            &mut job_spinners,
            &mut job_messages,
        );

        render_job_status(
            "创建快捷链接",
            &mut panel,
            &mut job_spinners,
            &mut job_messages,
        );

        render_job_status(
            "修正Shell环境脚本、matplotlib中文等问题",
            &mut panel,
            &mut job_spinners,
            &mut job_messages,
        );

        // let mut job1_progress: Progress;

        panel.fixed(&Frame::default(), 10); // 间隔空行

        Frame::default();

        panel.end();

        Step4Tab {
            panel,
            sender,
            job_spinners,
            job_messages,
            installer: None,
        }
    }

    pub fn widget(&self) -> &Flex {
        &self.panel
    }

    pub fn start(&mut self, installer: Installer) {
        self.installer = Some(installer.clone());
        let collector = Step4Collector::new(self.sender.clone());

        let handle = tokio::runtime::Handle::current();

        std::thread::spawn(move || {
            // 在新线程内运行异步代码
            handle.block_on(step4_run(installer, collector));
        });
    }

    pub fn handle_message(&mut self, msg: Step4Message) {
        match msg {
            Step4Message::JobStart(job_idx) => {
                self.job_spinners[job_idx].start();
                let message_label = &mut self.job_messages[job_idx];
                message_label.set_label_color(MESSAGE_COLOR);
                message_label.redraw();
            }
            Step4Message::JobSuccess(job_idx) => {
                self.job_spinners[job_idx].success();
            }
            Step4Message::JobError(job_idx, err) => {
                self.job_spinners[job_idx].error();
                fltk::dialog::alert_default(&err);
            }
            Step4Message::JobMessage(job_idx, message) => {
                self.job_messages[job_idx].set_label(&message);
            }
            msg @ _ => {
                unimplemented!("unimplemented {msg:?}")
            }
        }
    }

    pub fn take_installer(&mut self) -> Installer {
        self.installer.take().unwrap()
    }
}

pub struct Step4Collector {
    job_idx: usize,
    sender: Sender<Message>,
    // logs: InstallerLogs,
}

impl Step4Collector {
    pub fn new(sender: Sender<Message>) -> Self {
        Step4Collector {
            job_idx: 0,
            sender,
            // logs,
        }
    }
}

impl Step4Collector {
    pub fn next_job(&mut self) {
        self.job_idx += 1;
    }

    pub fn job_start(&mut self) {
        self.send(Step4Message::JobStart(self.job_idx));
    }

    pub fn job_success(&mut self) {
        self.send(Step4Message::JobSuccess(self.job_idx));
    }

    pub fn job_error(&mut self, err: String) {
        self.send(Step4Message::JobError(self.job_idx, err));
    }

    pub fn done(&mut self, installer: Installer) {
        self.send(Step4Message::Done(installer));
    }

    fn send(&self, msg: Step4Message) {
        self.sender.send(Message::Step4(msg));
    }
}

impl StatusUpdate for Step4Collector {
    fn message(&self, msg: &str) {
        self.send(Step4Message::JobMessage(self.job_idx, msg.to_string()));
    }

    fn update_downloading(&self, _status: &DownloadingStats) {
        unimplemented!()
    }
}

pub async fn step4_run(mut installer: Installer, mut collector: Step4Collector) {
    use super::super::pyenv::{
        create_winlnk, fix_patches, offline_install_requirements, set_platform_info,
    };

    if installer.platform_tag.is_none() {
        if let Err(err) = set_platform_info(&mut installer) {
            collector.job_error(format!("获取系统平台信息发生错误: {err}"));
            return;
        }
    }

    collector.job_start();
    if let Err(err) = offline_install_requirements(&installer).await {
        collector.job_error(format!("本地安装程序包发生错误: {err}"));
        return;
    };
    // tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    collector.job_success();

    //------------------------------------------------------------------------
    collector.next_job();
    collector.job_start();
    // tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    if let Err(err) = create_winlnk(&installer, &installer.target_dir()) {
        collector.job_error(format!("创建快捷方式发生错误: {err}"));
        return;
    };
    collector.job_success();

    // ----------------------------------------------------------------------
    collector.next_job();
    collector.job_start();
    if let Err(err) = fix_patches(&installer) {
        collector.job_error(format!("修正配置发生错误: {err}"));
        return;
    };
    collector.job_success();

    collector.done(installer);
}
