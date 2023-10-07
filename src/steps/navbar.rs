use fltk::{
    enums::{Color, Font},
    frame::Frame,
    group::Flex,
    prelude::{GroupExt, WidgetExt},
};

use super::super::style::AppStyle;

pub struct PhaseNavBar {
    labels: Vec<Frame>,
    seps: Vec<Frame>,
    phase_state: Vec<PhaseState>,
    bold_font: Font,
    normal_font: Font,
    navbar_row: Flex,
}

pub enum PhaseState {
    Pending,
    Success,
    Activate,
    Failure,
}

impl PhaseNavBar {
    const PENDING_COLOR: Color = Color::from_rgb(200, 200, 200);
    const ACTIVATE_COLOR: Color = Color::from_rgb(113, 36, 107);
    const SUCCESS_COLOR: Color = Color::from_rgb(0, 128, 0);

    pub fn new(style: &AppStyle) -> Self {
        static PHASE_TITLES: [&str; 6] = [
            "选择安装目录",
            "安装Python",
            "下载程序包",
            "安装程序包",
            "配置环境",
            "完成",
        ];

        static PHASE_WIDTHS: [i32; 6] = [105, 100, 90, 90, 75, 30];

        use fltk::enums::Align;

        let mut navbar_row = Flex::default().row();
        navbar_row.set_margins(0, 30, 0, 10);

        let mut phase_state = Vec::new();
        let mut labels = Vec::new();
        let mut seps = Vec::new();
        for (i, t) in PHASE_TITLES.iter().enumerate() {
            //
            let title = format!("{}. {}", i + 1, t);
            let mut frame = Frame::default()
                .with_label(&title)
                .with_align(Align::Inside | Align::Left);
            frame.set_label_color(PhaseNavBar::PENDING_COLOR);
            frame.set_label_font(style.font_zh);
            navbar_row.fixed(&frame, PHASE_WIDTHS[i]);

            labels.push(frame);
            phase_state.push(PhaseState::Pending);

            if i < PHASE_TITLES.len() - 1 {
                let mut frame = Frame::default().with_label(">");
                frame.set_label_color(PhaseNavBar::PENDING_COLOR);
                navbar_row.fixed(&frame, 15);
                seps.push(frame);
            }
        }

        navbar_row.end();

        let mut obj = Self {
            labels,
            seps,
            phase_state,
            normal_font: style.font_zh,
            bold_font: style.font_bold_zh,
            navbar_row,
        };

        obj.set_activate(0);

        obj
    }

    pub fn navbar_row(&self) -> &Flex {
        &self.navbar_row
    }

    pub fn set_activate(&mut self, idx: i32) {
        let idx = idx as usize;
        if idx > 0 {
            let prev_idx = idx - 1;
            self.phase_state[prev_idx] = PhaseState::Success;
            let frame = &mut self.labels[prev_idx];
            frame.set_label_color(PhaseNavBar::SUCCESS_COLOR);
            frame.set_label_font(self.normal_font);

            self.seps[prev_idx].set_label_color(PhaseNavBar::SUCCESS_COLOR)
        }

        self.phase_state[idx as usize] = PhaseState::Activate;
        let frame = &mut self.labels[idx as usize];
        frame.set_label_color(PhaseNavBar::ACTIVATE_COLOR);
        frame.set_label_font(self.bold_font);
    }
}
