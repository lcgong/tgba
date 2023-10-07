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
pub enum Step6Message {
    Enter,
    Update,
    Done,
}

pub struct Step6Tab {
    c_no: usize,
    panel: Flex,
    sender: Sender<Message>,
}

impl Step6Tab {
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

        Step6Tab {
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

    pub fn handle_message(&mut self, msg: Step6Message) {
        println!("handle: {}", self.c_no);

        match msg {
            Step6Message::Enter => {
                let s = self.sender.clone();
                tokio::spawn(async move {
                    use std::time::Duration;
                    tokio::time::sleep(Duration::from_millis(2000)).await;
                    s.send(Message::Step6(Step6Message::Done));
                });
            }
            Step6Message::Update => {
                //
            }
            Step6Message::Done => {
                //
            }
        }
    }
}
