use fltk::{
    app::Sender,
    // button::Button,
    // enums::Align,
    frame::Frame,
    group::{Flex, Group},
    misc::Progress,
    prelude::{GroupExt, WidgetBase, WidgetExt},
};

use super::super::{myapp::Message, style::AppStyle};

#[derive(Debug)]
pub enum Step4Message {
    Enter,
    Update,
    Done,
}

pub struct Step4Tab {
    c_no: usize,
    panel: Flex,
    sender: Sender<Message>,
}

impl Step4Tab {
    pub fn new(
        group: &mut Group,
        style: &AppStyle,
        sender: Sender<Message>,
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

        Step4Tab {
            c_no: 2,
            panel,
            sender,
        }
    }

    pub fn widget(&self) -> &Flex {
        &self.panel
    }

    pub fn c(&self) {
        println!("c");
    }

    pub fn handle_message(&mut self, msg: Step4Message) {
        println!("handle: {}", self.c_no);

        match msg {
            Step4Message::Enter => {
                let s = self.sender.clone();
                tokio::spawn(async move {
                    use std::time::Duration;
                    tokio::time::sleep(Duration::from_millis(2000)).await;
                    s.send(Message::Step4(Step4Message::Done));
                });
            }
            Step4Message::Update => {
                //
            }
            Step4Message::Done => {
                //
            }
        }
    }
}
