use fltk::{
    app::{Receiver, Sender},
    button::Button,
    enums::Align,
    frame::Frame,
    group::{Flex, Group},
    misc::Progress,
    prelude::{GroupExt, WidgetBase, WidgetExt},
};

use super::super::{myapp::Message, style::AppStyle};

#[derive(Debug)]
pub enum Step3Message {
    Enter,
    Update,
    Done,
}

pub struct Step3Tab {
    c_no: usize,
    panel: Flex,
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl Step3Tab {
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

        let mut progress = Progress::default();
        progress.set_minimum(0.0);
        progress.set_maximum(100.0);
        progress.set_selection_color(style.tgu_color);
        panel.fixed(&progress, 10);

        let frame = Frame::default();
        panel.fixed(&frame, 30);

        Frame::default();

        panel.end();

        Step3Tab {
            c_no: 2,
            panel,
            sender,
            receiver,
        }
    }

    pub fn widget(&self) -> &Flex {
        &self.panel
    }

    pub fn c(&self) {
        println!("c");
    }

    pub fn handle_message(&mut self, msg: Step3Message) {
        println!("handle: {}", self.c_no);

        match msg {
            Step3Message::Enter => {
                let s = self.sender.clone();
                tokio::spawn(async move {
                    use std::time::Duration;
                    tokio::time::sleep(Duration::from_millis(2000)).await;
                    s.send(Message::Step3(Step3Message::Done));
                });
            }
            Step3Message::Update => {
                //
            }
            Step3Message::Done => {
                //
            }
        }
    }
}
