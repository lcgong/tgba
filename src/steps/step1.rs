use std::ffi::OsString;
use std::{collections::HashMap, path::PathBuf};

use fltk::{
    app::Sender,
    button::Button,
    dialog::{FileDialog, FileDialogOptions, FileDialogType},
    enums::{Align, Color},
    frame::Frame,
    group::{Flex, Group},
    prelude::{GroupExt, InputExt, WidgetBase, WidgetExt},
};

use super::super::myapp::InstallerLogs;
use super::super::{myapp::Message, style::AppStyle};

use fltk::input::Input;

#[derive(Debug)]
pub enum Step1Message {
    Enter,
    Modified,
    Done { target_dir: String },
}

pub struct Step1Tab {
    panel: Flex,
    target_dir_input: Input,
    start_btn: Button,
    sender: Sender<Message>,
    logs: InstallerLogs,
}

impl Step1Tab {
    const DEFAUL_TARGET_DIR: &str = r#"C:\tgba"#;

    pub fn new(
        logs: InstallerLogs,
        group: &mut Group,
        style: &AppStyle,
        sender: Sender<Message>,
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

            let mut label = Frame::default()
                .with_label("安装目标目录：")
                .with_align(Align::Inside | Align::Left);
            label.set_label_font(style.font_zh);
            input_row.fixed(&label, 110);

            target_dir_input = Input::default();
            target_dir_input.set_value(Step1Tab::DEFAUL_TARGET_DIR);
            // target_dir_input.set_text_font(style.font_bold_en);
            target_dir_input.set_text_size(16);

            choose_btn = Button::default().with_label("选择..");
            choose_btn.set_label_font(style.font_bold_zh);
            choose_btn.set_label_color(style.darkgrey);
            choose_btn.set_size(60, choose_btn.height());
            input_row.fixed(&choose_btn, 60);

            input_row.end();
        }

        let mut hints_label;
        let mut hints_row = Flex::default().row();
        hints_row.set_margins(0, 0, 0, 0);
        {
            panel.fixed(&hints_row, 12);

            let label = Frame::default();
            hints_row.fixed(&label, 110);

            hints_label = Frame::default().with_align(Align::Inside | Align::Left);
            hints_label.set_label_color(style.darkgrey);

            let label = Frame::default();
            hints_row.fixed(&label, 60);

            hints_row.end();
        }

        let frame = Frame::default();
        panel.fixed(&frame, 30);

        let mut btn_row = Flex::default().row();
        {
            panel.fixed(&btn_row, 30);

            Frame::default();

            start_btn = Button::default().with_label("开始安装");
            start_btn.set_label_font(style.font_bold_zh);
            btn_row.fixed(&start_btn, 120);

            let frame = Frame::default();
            btn_row.fixed(&frame, 60);

            btn_row.end()
        }

        Frame::default();

        panel.end();

        let mut obj = Step1Tab {
            logs,
            panel,
            start_btn,
            target_dir_input,
            sender,
            // receiver,
        };

        {
            let disk_freespace = create_available_space_map();
            let mut input = obj.target_dir_input.clone();
            let mut hints_label = hints_label.clone();
            let mut start_btn = obj.start_btn.clone();

            if !check_availabel_space(&mut hints_label, &input.value(), &disk_freespace) {
                start_btn.deactivate();
            } else {
                start_btn.activate();
            }

            choose_btn.set_callback(move |_| {
                use FileDialogType::BrowseSaveDir;
                let mut dlg = FileDialog::new(BrowseSaveDir);

                let path = PathBuf::from(input.value());

                dlg.set_directory(&path).unwrap();
                dlg.set_option(FileDialogOptions::NewFolder);
                dlg.show();

                if !dlg.filename().as_os_str().is_empty() {
                    input.set_value(dlg.filename().to_string_lossy().as_ref());
                    if !check_availabel_space(&mut hints_label, &input.value(), &disk_freespace) {
                        start_btn.deactivate();
                    } else {
                        start_btn.activate();
                    }
                }
            });
        }

        let s = obj.sender.clone();
        obj.start_btn.set_callback({
            let input = obj.target_dir_input.clone();
            move |_| {
                s.send(Message::Step1(Step1Message::Done {
                    target_dir: input.value().to_string(),
                }))
            }
        });

        obj
    }

    pub fn widget(&self) -> &Flex {
        &self.panel
    }

    pub fn handle_message(&mut self, msg: Step1Message) {
        // println!("handle: {} msg: {:?}", self.a_no, msg);
    }
}

fn check_availabel_space(
    hints_label: &mut Frame,
    path: &str,
    map: &HashMap<OsString, f32>,
) -> bool {
    let path = PathBuf::from(path);
    // let hints = check_availabel_space(&disk_freespace, &path);

    use std::path::Component::Prefix;

    let expected_space = 2.5f32;

    if let Some(Prefix(prefix)) = path.components().next() {
        let driver = prefix.as_os_str().to_os_string();
        if let Some(freespace) = map.get(&driver) {
            if expected_space < *freespace {
                hints_label.set_label(&format!(
                    "安装所需空间约 {} GiB 内，剩余空间 {:.1} GiB",
                    expected_space, freespace
                ));
                hints_label.set_label_color(Color::from_rgb(5, 100, 5));

                return true;
            } else {
                hints_label.set_label(&format!(
                    "存储空间不足，安装所需空间约 {} GiB 内，剩余空间 {:.1} GiB",
                    expected_space, freespace
                ));
                hints_label.set_label_color(Color::from_rgb(200, 0, 0));

                return false;
            }
        }
    }

    hints_label.set_label(&format!("安装所需空间约 {} GiB 内", expected_space));
    hints_label.set_label_color(Color::from_rgb(150, 150, 150));
    return true;
}

fn create_available_space_map() -> HashMap<OsString, f32> {
    use std::path::Component;
    use sysinfo::{DiskExt, System, SystemExt};
    let mut available_space: HashMap<OsString, f32> = HashMap::new();
    for disk in System::new_all().disks() {
        match disk.mount_point().components().next() {
            Some(Component::Prefix(prefix_component)) => {
                let driver = prefix_component.as_os_str().to_os_string();
                let free = disk.available_space() as f32 / (2.0 as f32).powf(30.0);
                available_space.insert(driver, free);
            }
            _ => {}
        }
    }

    // println!("{:?}", available_space);
    available_space
}
