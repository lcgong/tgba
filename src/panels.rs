use fltk::{
    button::Button,
    frame::Frame,
    group::Flex,
    prelude::{GroupExt, WidgetBase, WidgetExt},
};






pub enum InstallerPanel {
    SettingsPanel(SettingsTab),
}

impl InstallerPanel {
    pub fn new_settings_panel() -> InstallerPanel {
        InstallerPanel::SettingsPanel(SettingsTab::new())
    }

    pub fn container(&self) -> &Flex {
        match self {
            InstallerPanel::SettingsPanel(panel) => panel.container(),
        }
    }
}

pub struct SettingsTab {
    panel: Flex,
    start_btn: Button,
}

impl SettingsTab {
    pub fn new() -> Self {
        use fltk::enums::Align;
        use fltk::input::Input;
        let mut panel = Flex::default_fill().column();
        panel.set_margins(0, 20, 20, 20);
        // parent.add(&panel);

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

        start_btn.set_callback(|_| {
            //
        });

        SettingsTab { panel, start_btn }
    }

    pub fn container(&self) -> &Flex {
        &self.panel
    }

    pub fn handle(&mut self) {
        //
    }
}
