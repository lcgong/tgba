use std::time::{Duration, Instant};

pub trait StatusUpdate {
    fn alert(&self, err: &str);
    fn message(&self, msg: &str);

    fn update_progress(&self, value: f64);
    fn update_downloading(&self, status: &DownloadingStats);
}

pub struct DownloadingStats {
    title: String,
    count: u64,
    start_time: Instant,
    total_size: u64,
    downloaded: u64,
    elasped: Option<Duration>,
    prev_start_time: Instant,
    prev_downloaded: u64,
}

impl DownloadingStats {
    pub fn new(title: &str, total_size: u64) -> Self {
        DownloadingStats {
            // installer,
            title: title.to_string(),
            count: 0,
            start_time: Instant::now(),
            total_size,
            downloaded: 0,
            elasped: None,
            prev_start_time: Instant::now(),
            prev_downloaded: 0,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn total_size(&self) -> u64 {
        self.total_size
    }

    pub fn out_of_tick(&self) -> bool {
        let elapsed = self
            .start_time
            .duration_since(self.prev_start_time)
            .as_secs_f64();

        elapsed > 0.5
    }

    pub fn next_tick(&mut self) {
        self.prev_start_time = self.start_time;
        self.prev_downloaded = self.downloaded;
    }

    pub fn update(&mut self, size: u64) {
        self.count += 1;
        self.downloaded += size;
        self.start_time = Instant::now();
    }

    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn downloaded(&self) -> u64 {
        self.downloaded
    }

    pub fn percentage(&self) -> f64 {
        if self.total_size != 0 {
            self.downloaded as f64 / (self.total_size as f64) * 100.0
        } else {
            0.0
        }
    }

    pub fn speed(&self) -> f64 {
        let downloaded = self.downloaded - self.prev_downloaded;
        let elapsed = self
            .start_time
            .duration_since(self.prev_start_time)
            .as_secs_f64();

        if elapsed.abs() < 1e-10 {
            0.0
        } else {
            downloaded as f64 / elapsed
        }
    }

    pub fn finish(&mut self) {
        self.elasped = Some(self.start_time.elapsed());
    }
}

use fltk::{
    frame::Frame,
    image::SvgImage,
    prelude::{ImageExt, WidgetExt},
};
use std::sync::{Arc, Mutex};

pub struct LoadingSpinner {
    // icon_empty: SvgImage,
    icon_success: SvgImage,
    icon_error: SvgImage,
    loading_frames: Vec<SvgImage>,
    frame: Frame,
    running: Arc<Mutex<bool>>,
    frame_idx: Arc<Mutex<u8>>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl LoadingSpinner {
    pub fn new(width: i32) -> Self {
        let (width, height) = (width, width);
        let (icon_empty, loading_frames) = create_loading_images(width, height);

        let icon_success = create_success_image(width, height);
        let icon_error = create_error_image(width, height);

        let mut frame = Frame::default();
        frame.set_image(Some(icon_empty.clone()));

        LoadingSpinner {
            // icon_empty,
            icon_success,
            icon_error,
            loading_frames,
            frame,
            running: Arc::new(Mutex::new(false)),
            frame_idx: Arc::new(Mutex::new(0)),
            handle: None,
        }
    }

    pub fn widget(&self) -> &Frame {
        &self.frame
    }

    pub fn start(&mut self) {
        let frame_images = self.loading_frames.clone();
        let frame = self.frame.clone();
        let running = Arc::clone(&self.running);
        let frame_idx = Arc::clone(&self.frame_idx);
        let handle = std::thread::spawn(move || {
            *running.lock().unwrap() = true;
            while *running.lock().unwrap() {
                let frame_idx = {
                    let mut frame_idx = frame_idx.lock().unwrap();
                    let idx = *frame_idx;
                    *frame_idx = (idx + 1) % 8;
                    idx
                };

                fltk::app::awake_callback({
                    let mut frame = frame.clone();
                    let image = frame_images[frame_idx as usize].clone();
                    move || {
                        frame.set_image(Some(image.clone()));
                        frame.redraw();
                    }
                });

                std::thread::sleep(Duration::from_millis(100));
            }
        });

        self.handle = Some(handle);
    }

    fn stop(&mut self) {
        if let Some(handle) = self.handle.take() {
            *self.running.lock().unwrap() = false;
            handle.join().unwrap();
        }
    }

    pub fn success(&mut self) {
        self.stop();
        self.frame.set_image(Some(self.icon_success.clone()));
        self.frame.redraw();
    }

    pub fn error(&mut self) {
        self.stop();
        self.frame.set_image(Some(self.icon_error.clone()));
        self.frame.redraw();
    }
}

fn create_success_image(width: i32, height: i32) -> SvgImage {
    let color = "rgb(0, 128, 0)"; // green color
    let mut image = loading_image(&[color; 8]);
    image.scale(width, height, true, true);
    image
}

fn create_error_image(width: i32, height: i32) -> SvgImage {
    let color = "rgb(200,  0, 0)"; // red color
    let mut image = loading_image(&[color; 8]);
    image.scale(width, height, true, true);
    image
}

fn create_loading_images(width: i32, height: i32) -> (SvgImage, Vec<SvgImage>) {
    // let col0 = "rgb(159, 194, 240)"; // color of background block
    let col0 = "rgb(200, 200, 200)";
    let col1 = "rgb(133,  25, 160)"; // color of highlight block
    let mut cols = [col0; 8];

    let mut img_empty = loading_image(&cols);
    img_empty.scale(width, height, true, true);

    let mut frames = Vec::new();
    for i in 0..8 {
        cols[i] = col1;

        let mut img = loading_image(&cols);
        img.scale(width, height, true, true);
        frames.push(img);

        cols[i] = col0;
    }

    (img_empty, frames)
}

fn loading_image(cols: &[&str; 8]) -> SvgImage {
    SvgImage::from_data(&format!(
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
    .unwrap()
}
