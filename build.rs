use winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon(r#"resources\tgba-jupyterlab-48x48.ico"#);
        res.compile().unwrap();
    }
}
