use fltk::{
    app::Sender,
    button::Button,
    enums::Align,
    frame::Frame,
    group::{Flex, Group},
    prelude::{GroupExt, WidgetBase, WidgetExt},
};

use super::super::{myapp::Message, pyenv::Installer, style::AppStyle};

#[derive(Debug)]
pub enum Step5Message {
    Enter(Installer),
    Done,
}

pub struct Step5Tab {
    panel: Flex,
    installer: Option<Installer>,
    _sender: Sender<Message>,
}

impl Step5Tab {
    pub fn new(group: &mut Group, _style: &AppStyle, sender: Sender<Message>) -> Self {
        let mut panel = Flex::default_fill().column();

        panel.resize(group.x(), group.y(), group.w(), group.h());
        group.add(&panel);

        panel.set_margins(0, 20, 20, 20);

        Frame::default();

        let mut message = Frame::default()
            .with_label("恭喜，TGBA实验平台已安装完成")
            .with_align(Align::Inside | Align::Center);
        message.set_label_size(20);

        panel.fixed(&Frame::default(), 25);

        let mut btn_flex = Flex::default_fill().row();
        Frame::default();
        let mut done_btn = Button::default().with_label("关闭安装程序");
        // done_btn.set_frame(fltk::enums::FrameType::ShadowFrame);
        done_btn.set_label_size(18);
        btn_flex.fixed(&done_btn, 120);
        Frame::default();
        btn_flex.end();

        panel.fixed(&btn_flex, 35);

        done_btn.set_callback(|_| {
            fltk::app::quit();
        });

        Frame::default();

        panel.end();

        Step5Tab {
            panel,
            installer: None,
            _sender: sender,
        }
    }

    pub fn widget(&self) -> &Flex {
        &self.panel
    }

    pub fn start(&mut self, installer: Installer) {
        self.installer = Some(installer.clone());
        // let collector = Step4Collector::new(self.logs.clone(), self.sender.clone());

        // let handle = tokio::runtime::Handle::current();
        // std::thread::spawn(move || {
        //     // 在新线程内运行异步代码
        //     handle.block_on(step5_run(installer, collector));
        // });
    }

    pub fn handle_message(&mut self, msg: Step5Message) {
        match msg {
            msg @ _ => {
                unimplemented!("{msg:?}")
            }
        }
    }
}
