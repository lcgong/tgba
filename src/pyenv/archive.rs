use anyhow::{bail, Result};
use sha2::{Digest, Sha256};
use std::path::Path;

pub fn unpack_archive(extension: &str, bytes: &[u8], dest: &Path) -> Result<()> {
    match extension {
        ".zip" => unpack_zip(bytes, dest)?,
        ".tar.zstd" => {
            unpack_tar(zstd::stream::read::Decoder::with_buffer(bytes)?, dest)?;
        }
        ".tar.gz" | ".tgz" => {
            unpack_tar(flate2::bufread::GzDecoder::new(bytes), dest)?;
        }
        ".tar.bz2" | ".tbz" => {
            unpack_tar(bzip2::bufread::BzDecoder::new(bytes), dest)?;
        }
        ".tar.xz" | ".txz" | ".tlz" | ".tar.lz" | ".tar.lzma" => {
            bail!("unimplemented archive extension: '{}'", extension);
        }
        _ => {
            bail!("unknown archive extension: '{}'", extension);
        }
    }

    Ok(())
}



fn unpack_zip(bytes: &[u8], dest: &Path) -> Result<()> {
    use std::io::Cursor;
    use zip::read::ZipArchive;

    let strip_components = 1;

    let mut archive = ZipArchive::new(Cursor::new(bytes))?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        let Some(name) = file.enclosed_name() else {
            bail!("Invalid file path in zip")
        };

        let mut components = name.components();
        for _ in 0..strip_components {
            components.next();
        }

        let path = dest.join(components.as_path());
        if path != Path::new("") && path.strip_prefix(dest).is_ok() {
            if file.name().ends_with('/') {
                std::fs::create_dir_all(&path)?;
            } else {
                if let Some(p) = path.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p)?;
                    }
                }
                std::io::copy(&mut file, &mut std::fs::File::create(&path)?)?;
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

    Ok(())
}

fn unpack_tar<R: std::io::Read>(obj: R, dest: &Path) -> Result<()> {
    let strip_components = 1;

    let mut archive = tar::Archive::new(obj);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let name = entry.path()?;

        let mut components = name.components();

        for _ in 0..strip_components {
            components.next();
        }

        let dst_path = dest.join(components.as_path());

        // only unpack if it's save to do so
        if dst_path != Path::new("") && dst_path.strip_prefix(dest).is_ok() {
            if let Some(dir) = dst_path.parent() {
                std::fs::create_dir_all(dir).ok();
            }
            entry.unpack(&dst_path)?;
        }
    }

    Ok(())
}

pub fn checksum(method: &str, content: &[u8], hexcode:&str) -> Result<bool> {
    match method.to_lowercase().as_str() {
        "sha256" => Ok(sha256_checksum(content, hexcode)),
        _ => bail!("不支持checksum方法: {}", method)
    }
}

/// Takes a bytes slice and compares it to a given string checksum.
fn sha256_checksum(content: &[u8], hexcode: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(content);

    let digest = hasher.finalize();
    
    let digest = hex::encode(digest);
    if !digest.eq_ignore_ascii_case(hexcode) {
        false
    } else {
        true
    }
}
