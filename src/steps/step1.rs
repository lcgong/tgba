use fltk::{
    app::{Receiver, Sender},
    button::Button,
    enums::Align,
    frame::Frame,
    group::{Flex, Group},
    prelude::{GroupExt, WidgetBase, WidgetExt},
};

use super::super::{myapp::Message, style::AppStyle};

// myapp.rs(136, 57): unexpected argument of type `fltk::app::`

use fltk::input::Input;

#[derive(Debug)]
pub enum Step1Message {
    Enter,
    Modified,
    Done,
}

pub struct Step1Tab {
    a_no: usize,
    panel: Flex,
    start_btn: Button,
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl Step1Tab {
    pub fn a(&self) {
        println!("a");
    }

    pub fn new(
        group: &mut Group,
        style: &AppStyle,
        sender: Sender<Message>,
        receiver: Receiver<Message>,
    ) -> Self {
        let mut panel = Flex::default_fill().column();

        panel.resize(group.x(), group.y(), group.w(), group.h());
        group.add(&panel);

        panel.set_margins(0, 20, 20, 20);

        Frame::default();

        let mut choose_btn: Button;
        let mut start_btn: Button;
        let mut target_dir_input: Input;

        let mut input_row = Flex::default().row();
        {
            panel.fixed(&input_row, 30);

            let label = Frame::default()
                .with_label("安装到的目标目录：")
                .with_align(Align::Inside | Align::Left);
            input_row.fixed(&label, 130);

            target_dir_input = Input::default();

            choose_btn = Button::default().with_label("选择..");
            input_row.fixed(&choose_btn, 60);

            input_row.end();
        }

        let frame = Frame::default();
        panel.fixed(&frame, 30);

        let mut btn_row = Flex::default().row();
        {
            panel.fixed(&btn_row, 30);

            Frame::default();

            start_btn = Button::default().with_label("开始安装");
            btn_row.fixed(&start_btn, 120);

            let frame = Frame::default();
            btn_row.fixed(&frame, 60);

            btn_row.end()
        }

        Frame::default();

        panel.end();

        choose_btn.set_callback(|_| {
            //
        });

        let s = sender.clone();
        start_btn.set_callback(move |_| s.send(Message::Step1(Step1Message::Done)));

        Step1Tab {
            a_no: 0,
            panel,
            start_btn,
            sender,
            receiver,
        }
    }

    pub fn widget(&self) -> &Flex {
        &self.panel
    }

    pub fn handle_message(&mut self, msg: Step1Message) {
        println!("handle: {} msg: {:?}", self.a_no, msg);
    }
}
