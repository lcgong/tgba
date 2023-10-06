// use fltk::{
//     button::Button,
//     frame::Frame,
//     group::Flex,
//     prelude::{GroupExt, WidgetBase, WidgetExt},
// };


#[derive(Debug)]
pub enum Step3Message {
    Enter,
}

pub struct Step3Tab {
    c_no: usize,
}

impl Step3Tab {
    pub fn new() -> Self {
        Step3Tab {
            c_no: 2
        }
    }

    pub fn c(&self) {
        println!("c");
    }

    pub fn handle_message(&mut self) {
        println!("handle: {}", self.c_no);
    }
}
