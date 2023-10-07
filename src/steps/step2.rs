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
pub enum Step2Message {
    Enter,
    Update,
    Done,
}

pub struct Step2Tab {
    b_no: usize,
    panel: Flex,
    sender: Sender<Message>,
}

impl Step2Tab {
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
        progress.set_color(fltk::enums::Color::from_rgb(200, 200, 200));
        progress.set_frame(fltk::enums::FrameType::FlatBox);

        progress.set_minimum(0.0);
        progress.set_maximum(100.0);
        progress.set_selection_color(style.tgu_color);
        panel.fixed(&progress, 5);

        let frame = Frame::default();
        panel.fixed(&frame, 30);

        Frame::default();

        panel.end();


        Step2Tab {
            b_no: 1,
            panel,
            sender,
        }
    }

    pub fn widget(&self) -> &Flex {
        &self.panel
    }

    pub fn b(&self) {
        println!("b");
    }

    pub fn handle_message(&mut self, msg: Step2Message) {
        println!("handle: {}", self.b_no);

        match msg {
            Step2Message::Enter => {
                let s = self.sender.clone();
                tokio::spawn(async move {
                    use std::time::Duration;
                    tokio::time::sleep(Duration::from_millis(2000)).await;
                    s.send(Message::Step2(Step2Message::Done));
                });
        
            },
            Step2Message::Update => {
                //
            },
            Step2Message::Done => {
                //
            },
        }
    }
}
