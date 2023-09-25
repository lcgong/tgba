use anyhow::{anyhow, bail, Context, Error};
use sha2::{Digest, Sha256};
use std::borrow::Cow;
use std::env::consts::{ARCH, OS};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
static ECHO_TO_STDERR: AtomicBool = AtomicBool::new(false);

use super::version::PythonVersion;

pub static PYTHON_VERSION: PythonVersion = PythonVersion {
    kind: Cow::Borrowed("cpython"),
    major: 3,
    minor: 11,
    patch: 5,
    suffix: None,
};

pub fn main() {
    ensure_venv(CommandOutput::Verbose).unwrap();
}

#[doc(hidden)]
pub fn _print(args: std::fmt::Arguments) {
    // use eprintln and println so that tests can still intercept this
    if ECHO_TO_STDERR.load(Ordering::Relaxed) {
        eprintln!("{}", args);
    } else {
        println!("{}", args);
    }
}

/// Echo a line to the output stream (usually stdout).
macro_rules! echo {
    () => {
        $crate::tui::_print(format_args!(""))
    };
    ($($arg:tt)+) => {
        // TODO: this is bloaty, but this way capturing of outputs
        // for stdout works in tests still.
        _print(format_args!($($arg)*))
    }
}

/// Controls the fetch output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum CommandOutput {
    /// Regular output
    #[default]
    Normal,
    /// Extra verbose output
    Verbose,
    /// No output
    Quiet,
}

pub fn ensure_venv(output: CommandOutput) -> Result<PathBuf, Error> {
    let app_dir = get_app_path()?;
    let venv_dir = app_dir.join("venv");

    ensure_python_dist(&PYTHON_VERSION, output)?;

    let python_bin = get_python_bin(&PYTHON_VERSION)?;

    // initialize the virtualenv
    let mut venv_cmd = Command::new(&python_bin);
    venv_cmd.arg("-mvenv");
    venv_cmd.arg(&venv_dir);

    let status = venv_cmd
        .status()
        .with_context(|| format!("unable to create self venv using {}", python_bin.display()))?;
    if !status.success() {
        bail!("failed to initialize virtualenv in {}", venv_dir.display());
    }

    Ok(venv_dir)
}

pub fn ensure_python_dist(version: &PythonVersion, output: CommandOutput) -> Result<(), Error> {
    let py_dir = get_canonical_py_path(&version)?;
    let py_bin = get_python_bin(&version)?;
    if py_dir.is_dir() && py_bin.is_file() {
        if output == CommandOutput::Verbose {
            echo!("Python version already downloaded. Skipping.");
        }
        return Ok(());
    }

    let (url, sha256) = match get_download_url(version, OS, ARCH) {
        Some(result) => result,
        None => bail!("unknown version {}", version),
    };

    if output == CommandOutput::Verbose {
        echo!("target dir: {}", py_dir.display());
    }

    fs::create_dir_all(&py_dir)
        .with_context(|| format!("failed to create target folder {}", py_dir.display()))?;

    if output == CommandOutput::Verbose {
        echo!("download url: {}", url);
    }
    if output != CommandOutput::Quiet {
        echo!("{} {}", "Downloading", version);
    }
    let archive_buffer = download_url(url, output)?;

    if let Some(sha256) = sha256 {
        if output != CommandOutput::Quiet {
            echo!("{}", "Checking checksum");
        }
        check_checksum(&archive_buffer, sha256)
            .with_context(|| format!("hash check of {} failed", &url))?;
    } else if output != CommandOutput::Quiet {
        echo!("Checksum check skipped (no hash available)");
    }

    unpack_archive(&archive_buffer, &py_dir, 1)
        .with_context(|| format!("unpacking of downloaded tarball {} failed", &url))?;

    if output != CommandOutput::Quiet {
        echo!("{} Downloaded {}", "success:", version);
    }

    Ok(())
}

/// Takes a bytes slice and compares it to a given string checksum.
pub fn check_checksum(content: &[u8], checksum: &str) -> Result<(), Error> {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let digest = hasher.finalize();
    let digest = hex::encode(digest);
    if !digest.eq_ignore_ascii_case(checksum) {
        bail!("hash mismatch: expected {} got {}", checksum, digest);
    }
    Ok(())
}

pub fn matches_version(req: &PythonVersion, v: &PythonVersion) -> bool {
    if req.kind != v.kind {
        return false;
    }
    if req.major != v.major {
        return false;
    }
    if req.minor != v.minor {
        return false;
    }
    if req.patch != v.patch {
        return false;
    }
    if let Some(ref suffix) = req.suffix {
        if Some(suffix) != v.suffix.as_ref() {
            return false;
        }
    }
    true
}

/// Given a version, platform and architecture returns the download URL.
pub fn get_download_url(
    version: &PythonVersion,
    platform: &str,
    arch: &str,
) -> Option<(&'static str, Option<&'static str>)> {
    for (it_version, it_arch, it_platform, it_url, it_sha256) in super::dists::PYTHON_VERSIONS {
        if platform == *it_platform && arch == *it_arch && matches_version(version, it_version) {
            return Some((it_url, *it_sha256));
        }
    }
    None
}

pub fn download_url(url: &str, output: CommandOutput) -> Result<Vec<u8>, Error> {
    // for now we only allow HTTPS downloads.
    if !url.starts_with("https://") {
        bail!("Refusing insecure download");
    }

    let mut archive_buffer = Vec::new();
    let mut handle = curl::easy::Easy::new();
    handle.url(url)?;
    handle.progress(true)?;
    handle.follow_location(true)?;

    let write_archive = &mut archive_buffer;
    {
        let mut transfer = handle.transfer();

        let mut count = 0;
        transfer.progress_function(move |a, b, _, _| {
            let (down_len, down_pos) = (a as u64, b as u64);
            if down_len > 0 {
                if count >= 50 {
                    count = 0;
                    print!(".");
                    std::io::stdout().flush().unwrap();
                } else {
                    count += 1;
                }
            }
            true
        })?;

        use std::io::Write;

        transfer.write_function(move |data| {
            write_archive.write_all(data).unwrap();
            Ok(data.len())
        })?;
        transfer
            .perform()
            .with_context(|| format!("download of {} failed", &url))?;
    }

    let code = handle.response_code()?;
    if code == 404 {
        // Ok(None)
        bail!("Failed to download: 404 not found")
    } else if !(200..300).contains(&code) {
        bail!("Failed to download: {}", code)
    } else {
        Ok(archive_buffer)
    }
}

/// Returns the path of the python binary for the given version.
pub fn get_python_bin(version: &PythonVersion) -> Result<PathBuf, Error> {
    let mut p = get_canonical_py_path(version)?;

    // It's permissible to link Python binaries directly in two ways.  It can either be
    // a symlink in which case it's used directly, it can be a non-executable text file
    // in which case the contents are the location of the interpreter, or it can be an
    // executable file on unix.
    if p.is_file() {
        if p.is_symlink() {
            return Ok(p.canonicalize()?);
        }
        #[cfg(unix)]
        {
            use std::os::unix::prelude::MetadataExt;
            if p.metadata().map_or(false, |x| x.mode() & 0o001 != 0) {
                return Ok(p);
            }
        }
        let contents = fs::read_to_string(&p).context("could not read toolchain file")?;
        return Ok(PathBuf::from(contents.trim_end()));
    }

    // we support install/bin/python, install/python and bin/python
    p.push("install");
    if !p.is_dir() {
        p.pop();
    }
    p.push("bin");
    if !p.is_dir() {
        p.pop();
    }

    #[cfg(unix)]
    {
        p.push("python3");
    }
    #[cfg(windows)]
    {
        p.push("python.exe");
    }

    Ok(p)
}

pub fn get_app_path() -> Result<PathBuf, Error> {
    let rv = std::env::current_dir().map(|x| x.join(".tgba"))?;
    Ok(rv)
}

pub fn get_canonical_py_path(version: &PythonVersion) -> Result<PathBuf, Error> {
    let mut rv = get_app_path()?;
    rv.push(version.to_string());
    Ok(rv)
}

#[derive(Copy, Clone, Debug)]
enum ArchiveFormat {
    // TarGz,
    // TarBz2,
    TarZstd,
    Zip,
}

impl ArchiveFormat {
    pub fn peek(bytes: &[u8]) -> Option<ArchiveFormat> {
        use std::io::Read;

        let mut buf = [0u8; 1];
        if zstd::stream::read::Decoder::with_buffer(bytes)
            .map_or(false, |x| x.single_frame().read(&mut buf).is_ok())
        {
            Some(ArchiveFormat::TarZstd)
        // } else if flate2::bufread::GzDecoder::new(bytes).header().is_some() {
        // Some(ArchiveFormat::TarGz)
        // } else if bzip2::bufread::BzDecoder::new(bytes).read(&mut buf).is_ok() {
        // Some(ArchiveFormat::TarBz2)
        } else if zip::read::ZipArchive::new(std::io::Cursor::new(bytes)).is_ok() {
            Some(ArchiveFormat::Zip)
        } else {
            None
        }
    }

    pub fn make_decoder<'a>(self, bytes: &'a [u8]) -> Result<Box<dyn std::io::Read + 'a>, Error> {
        Ok(match self {
            // ArchiveFormat::TarGz => Box::new(flate2::bufread::GzDecoder::new(bytes)) as Box<_>,
            // ArchiveFormat::TarBz2 => Box::new(bzip2::bufread::BzDecoder::new(bytes)) as Box<_>,
            ArchiveFormat::TarZstd => {
                Box::new(zstd::stream::read::Decoder::with_buffer(bytes)?) as Box<_>
            }
            ArchiveFormat::Zip => return Err(anyhow!("zip cannot be decoded with read")),
        })
    }
}

/// Unpacks a tarball or zip archive.
///
/// Today this assumes that the tarball is zstd compressed which happens
/// to be what the indygreg python builds use.
pub fn unpack_archive(contents: &[u8], dst: &Path, strip_components: usize) -> Result<(), Error> {
    let format = ArchiveFormat::peek(contents).ok_or_else(|| anyhow!("unknown archive"))?;

    if matches!(format, ArchiveFormat::Zip) {
        use std::io::Cursor;
        use zip::read::ZipArchive;

        let mut archive = ZipArchive::new(Cursor::new(contents))?;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let name = file
                .enclosed_name()
                .ok_or_else(|| anyhow!("Invalid file path in zip"))?;
            let mut components = name.components();
            for _ in 0..strip_components {
                components.next();
            }

            let path = dst.join(components.as_path());
            if path != Path::new("") && path.strip_prefix(dst).is_ok() {
                if file.name().ends_with('/') {
                    fs::create_dir_all(&path)?;
                } else {
                    if let Some(p) = path.parent() {
                        if !p.exists() {
                            fs::create_dir_all(p)?;
                        }
                    }
                    std::io::copy(&mut file, &mut fs::File::create(&path)?)?;
                }
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Some(mode) = file.unix_mode() {
                        fs::set_permissions(&path, fs::Permissions::from_mode(mode))?;
                    }
                }
            }
        }
    } else {
        let mut archive = tar::Archive::new(format.make_decoder(contents)?);
        for entry in archive.entries()? {
            let mut entry = entry?;
            let name = entry.path()?;
            let mut components = name.components();
            for _ in 0..strip_components {
                components.next();
            }
            let path = dst.join(components.as_path());

            // only unpack if it's save to do so
            if path != Path::new("") && path.strip_prefix(dst).is_ok() {
                if let Some(dir) = path.parent() {
                    fs::create_dir_all(dir).ok();
                }
                entry.unpack(&path)?;
            }
        }
    }

    Ok(())
}
