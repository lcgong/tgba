use fltk::{
    app::{Receiver, Sender},
    button::Button,
    enums::{Color, Event},
    frame::Frame,
    group::{Flex, Group},
    image::IcoImage,
    prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt},
    window::DoubleWindow,
};

pub fn show_dialog() -> MyDialog {
    MyDialog::default()
}

pub struct MyDialog {
    inp: fltk::input::Input,
}

impl MyDialog {
    pub fn default() -> Self {
        let mut win = DoubleWindow::default()
            .with_size(400, 100)
            .with_label("My Dialog");
        win.set_color(Color::from_rgb(240, 240, 240));
        let mut pack = fltk::group::Pack::default()
            .with_size(300, 30)
            .center_of_parent()
            .with_type(fltk::group::PackType::Horizontal);
        pack.set_spacing(20);
        fltk::frame::Frame::default()
            .with_size(80, 0)
            .with_label("Enter name:");
        let mut inp = fltk::input::Input::default().with_size(100, 0);
        inp.set_frame(fltk::enums::FrameType::FlatBox);
        let mut ok = fltk::button::Button::default()
            .with_size(80, 0)
            .with_label("Ok");
        style_button(&mut ok);
        pack.end();
        win.end();
        win.make_modal(true);
        win.show();
        ok.set_callback({
            let mut win = win.clone();
            move |_| {
                win.hide();
            }
        });
        while win.shown() {
            fltk::app::wait();
        }
        Self { inp }
    }
    // pub fn value(&self) -> String {
    //     self.inp.value()
    // }
}

fn style_button(btn: &mut fltk::button::Button) {
    btn.set_color(fltk::enums::Color::Cyan);
    btn.set_frame(fltk::enums::FrameType::RFlatBox);
    btn.clear_visible_focus();
}
