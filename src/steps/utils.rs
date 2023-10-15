

pub fn format_scale(size: f64, precision: usize) -> String {
    let scale_kb = 2u64.pow(10) as f64;
    if size < scale_kb {
        return format!("{size:.0}B");
    }

    let scale_mb = 2u64.pow(20) as f64;
    if size < scale_mb {
        let size = (size as f64) / (scale_kb as f64);
        return format!("{size:.*}KiB", precision);
    }

    let scale_gb = 2u64.pow(30) as f64;
    if size < scale_gb {
        let size = (size as f64) / (scale_mb as f64);
        return format!("{size:.*}MiB", precision);
    }

    let size = (size as f64) / (scale_gb as f64);
    return format!("{size:.*}GiB", precision);
}
