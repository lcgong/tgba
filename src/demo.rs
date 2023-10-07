// use std::path;

use fltk::app;
use fltk::button::Button;
// use fltk::enums::FrameType;
use fltk::frame::Frame;
use fltk::group::Flex;
use fltk::image::IcoImage;
use fltk::image::SvgImage;
use fltk::prelude::GroupExt;
use fltk::prelude::WindowExt;
use fltk::prelude::{ImageExt, WidgetBase, WidgetExt};
use fltk::window::DoubleWindow;

fn get_loading_images(width: i32, height: i32) -> Vec<SvgImage> {
    let col0 = "rgb(159, 194, 240)"; // color of background block
    let col1 = "rgb(133,  25, 160)"; // color of highlight block
    let mut cols = [col0; 8];

    let mut images = Vec::new();
    for i in 0..8 {
        cols[i] = col1;

        let mut img = SvgImage::from_data(&format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" 
    width="200px" height="200px" viewBox="0 0 100 100" 
    preserveAspectRatio="xMidYMid" style="display:block" >
<rect x="10" y="10" width="24" height="24" fill="{}" ></rect>
<rect x="38" y="10" width="24" height="24" fill="{}" ></rect>
<rect x="66" y="10" width="24" height="24" fill="{}" ></rect>
<rect x="66" y="38" width="24" height="24" fill="{}" ></rect>
<rect x="66" y="66" width="24" height="24" fill="{}" ></rect>
<rect x="38" y="66" width="24" height="24" fill="{}" ></rect>
<rect x="10" y="66" width="24" height="24" fill="{}" ></rect>
<rect x="10" y="38" width="24" height="24" fill="{}" ></rect>
</svg>"#,
            cols[0], cols[1], cols[2], cols[3], cols[4], cols[5], cols[6], cols[7]
        ))
        .unwrap();

        img.scale(width, height, true, true);
        images.push(img);

        cols[i] = col0;
    }

    images
}

pub async fn win_main() {
    let tgu_color = fltk::enums::Color::from_rgb(113, 36, 107);

    let icon: IcoImage =
        IcoImage::load(&std::path::Path::new("resources/tgba-jupyterlab-48x48.ico")).unwrap();
    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    app::background(255, 255, 255);

    let mut win = DoubleWindow::new(200, 200, 600, 400, "TGBA安装程序");
    win.set_icon(Some(icon));
    win.make_resizable(true);


    let mut chooser = fltk::dialog::FileChooser::new(
        ".",                    // directory
        "*",                    // filter or pattern
        fltk::dialog::FileChooserType::Directory, // chooser type
        "Title Of Chooser",     // title
    );

    chooser.set_preview(false);

    let mut col = Flex::default_fill().column();
    col.set_margins(20, 20, 20, 20);

    let mut image_frame = Frame::default().with_size(52, 52).center_of_parent();
    // frame.set_frame(FrameType::EngravedBox);

    let images = get_loading_images(48, 48);

    image_frame.set_image(Some(images[0].clone()));

    let mut progress = fltk::misc::Progress::default();
    progress.set_minimum(0.0);
    progress.set_maximum(100.0);
    progress.set_selection_color(tgu_color);
    col.fixed(&progress, 10);

    let mut frame = Frame::default();
    let mut but = Button::default().with_label("Click me!");
    col.fixed(&but, 40);

    let mut dir_but = Button::default().with_label("选择安装目录");
    col.fixed(&dir_but, 40);

    col.end();

    win.end();
    win.show();
    dir_but.set_callback(move |_| {
        let mode = fltk::dialog::FileDialogType::BrowseDir;
        let mut nfc = fltk::dialog::FileDialog::new(mode);
        use std::path::PathBuf;
        let path = PathBuf::from(r#"C:\"#);
        nfc.set_directory(&path).unwrap();
        nfc.set_option(fltk::dialog::FileDialogOptions::NewFolder);
        nfc.show();
        println!("select: {}", nfc.filename().display());
    });

    but.set_callback(move |_| {
        frame.set_label("Hello world");

        let frame = frame.clone();
        let progressbar = progress.clone();
        let mut image_frame = image_frame.clone();
        let images = images.clone();
        std::thread::spawn(move || {
            for i in 1..101 {
                use std::thread::sleep;
                use std::time::Duration;
                sleep(Duration::from_millis(100));
                let msg = format!("Hello world-{:3}", i);

                println!("{}", msg);
                let mut label_frame = frame.clone();
                let mut progress = progressbar.clone();
                image_frame.set_image(Some(images[i % 8].clone()));
                progress.set_value(i as f64);
                label_frame.set_label(msg.as_str());

                let mut image_frame = image_frame.clone();
                app::awake_callback(move || {
                    // 使用FLTK的事件循环机制刷新组件
                    image_frame.redraw();
                    label_frame.redraw();
                });
            }
        });

        // tokio::spawn(async move {
        //     for i in 1..11 {
        //         use tokio::time::{sleep, Duration};
        //         sleep(Duration::from_millis(1000)).await;
        //         let msg = format!("Hello world-{}", i);

        //         println!("{}", msg);
        //         let mut frame = frame.clone();
        //         app::awake_callback(move || {
        //             // 使用FLTK的事件循环机制刷新组件
        //             frame.set_label(msg.as_str());
        //             frame.redraw();
        //         });
        //     }
        // });
    });

    app.run().unwrap();
    println!("exit!");
}
