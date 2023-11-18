use std::sync::{Arc, Mutex};

use fltk::{
    button::Button,
    enums::Color,
    frame::Frame,
    group::Flex,
    prelude::{DisplayExt, GroupExt, WidgetBase, WidgetExt, WindowExt},
    text::{TextBuffer, TextDisplay},
    window::DoubleWindow,
};

fn center_of_window<Widget: WidgetExt, Window: WidgetExt>(win: &mut Widget, parent: &Window) {
    // println!(
    //     "[{}, {}] of [{},{}] @ {}, {}",
    //     win.width(),
    //     win.height(),
    //     parent.width(),
    //     parent.height(),
    //     parent.x(),
    //     parent.y()
    // );

    let (sw, sh) = (win.width(), win.height());
    let (pw, ph) = (parent.width(), parent.height());
    let x = ((pw - sw) as f32 / 2.0) as i32 + parent.x();
    let y = ((ph - sh) as f32 / 2.0) as i32 + parent.y();
    win.set_pos(x, y);
}

pub struct QuitConfirmDialog {}

impl QuitConfirmDialog {
    pub fn new<W: WidgetExt>(parent: &W) -> Self {
        let width = 400;
        let height = 120;

        // let x = fltk::app::event_x_root() - width / 2;
        // let y = fltk::app::event_y_root() - height / 2;

        let mut win = DoubleWindow::default()
            .with_size(width, height)
            .with_label("请确认");

        center_of_window(&mut win, parent);

        let mut body_flex = Flex::default_fill().column();
        body_flex.set_spacing(10);
        body_flex.set_margins(10, 20, 10, 20);

        let mut message_label = Frame::default();
        message_label.set_label("是否中断现在安装?");
        message_label.set_label_size(14);

        Frame::default();

        let mut btn1;
        let mut btn2;
        let mut btnbar_flex = Flex::default_fill().row();
        {
            body_flex.fixed(&btnbar_flex, 30);
            btnbar_flex.set_spacing(20);

            Frame::default();

            btn1 = Button::default();
            btn1.set_label("继续安装");
            btn1.set_label_size(14);
            btn1.clear_visible_focus();
            btnbar_flex.fixed(&btn1, 120);

            btn2 = Button::default();
            btn2.set_label("中断现在安装");
            btn2.set_label_size(14);
            btn2.clear_visible_focus();
            btn2.set_color(Color::from_rgb(200, 0, 0));
            btn2.set_label_color(Color::White);
            btnbar_flex.fixed(&btn2, 120);

            Frame::default();

            btnbar_flex.end();
        }

        body_flex.end();

        win.end();
        win.make_modal(true);
        win.show();

        btn1.set_callback({
            let mut win = win.clone();
            move |_| {
                win.hide();
            }
        });

        btn2.set_callback({
            let mut win = win.clone();
            move |_| {
                win.hide();
                fltk::app::quit();
            }
        });

        while win.shown() {
            fltk::app::wait();
        }
        Self {}
    }
}

pub fn confirm_quit_dialog<W: WidgetExt>(parent: &W) {
    QuitConfirmDialog::new(parent);
}

pub fn error_confirm<W: WidgetExt>(parent: &W, title: &str, message: &str) -> i32 {

    let (mut width, mut height) = fltk::draw::measure(message, true);
    width += 50;
    height += 100;

    use std::cmp::{max, min};

    width = min(max(width, 400), 800);
    height = min(max(height, 200), 300);

    let mut win = DoubleWindow::default()
        .with_size(width, height)
        .with_label(title);

    center_of_window(&mut win, parent);

    let mut body_flex = Flex::default_fill().column();
    body_flex.set_spacing(10);
    body_flex.set_margins(10, 20, 10, 20);

    // let mut title_label = Frame::default();
    // title_label.set_label(title);
    // title_label.set_label_size(14);
    // body_flex.fixed(&title_label, 16);

    let mut details_label = TextDisplay::default();
    let mut details_buf = TextBuffer::default();
    details_buf.set_text(message);
    details_label.set_buffer(details_buf);

    // Frame::default();

    let mut retry_btn;
    let mut exit_btn;

    let mut btnbar_flex = Flex::default_fill().row();
    {
        body_flex.fixed(&btnbar_flex, 30);
        btnbar_flex.set_spacing(20);

        Frame::default();

        retry_btn = Button::default();
        retry_btn.set_label("重试");
        retry_btn.set_label_size(14);
        retry_btn.clear_visible_focus();
        btnbar_flex.fixed(&retry_btn, 120);

        exit_btn = Button::default();
        exit_btn.set_label("放弃安装");
        exit_btn.set_label_size(14);
        exit_btn.clear_visible_focus();
        exit_btn.set_color(Color::from_rgb(200, 0, 0));
        exit_btn.set_label_color(Color::White);
        btnbar_flex.fixed(&exit_btn, 120);

        Frame::default();

        btnbar_flex.end();
    }

    body_flex.end();

    win.end();
    win.make_modal(true);
    win.show();

    let result = Arc::new(Mutex::new(0));

    retry_btn.set_callback({
        let mut win = win.clone();
        let result = result.clone();
        move |_| {
            win.hide();

            *result.lock().unwrap() = 1;
        }
    });

    exit_btn.set_callback({
        let mut win = win.clone();
        let result = result.clone();
        move |_| {
            win.hide();
            fltk::app::quit();
            *result.lock().unwrap() = 2;
        }
    });

    while win.shown() {
        fltk::app::wait();
    }

    return *result.lock().unwrap();
}
