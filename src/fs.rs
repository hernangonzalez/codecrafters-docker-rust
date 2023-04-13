use anyhow::{Context, Ok, Result};
use bytes::Bytes;
use flate2::read::GzDecoder;
use std::env;
use std::os::unix::{self, fs::PermissionsExt};
use std::{
    fs::{self, File, Permissions},
    io,
    path::Path,
};
use tar::Archive;
use tempfile::tempdir;

const RW_MODE: u32 = 0o666;

pub fn create_root() -> Result<Box<Path>> {
    let temp_dir = tempdir()?;
    Ok(temp_dir.into_path().into_boxed_path())
}

pub fn isolate(cmd: &Path) -> Result<()> {
    change_root(cmd)?;
    prepare_fs()?;
    start_namespace()?;
    Ok(())
}

pub fn unpack(bytes: Bytes, dst: &Path) -> Result<()> {
    let tar_path = dst.with_file_name("image.tar");
    deflate(bytes, &tar_path)?;

    let file = File::open(&tar_path)?;
    let mut archive = Archive::new(file);
    archive.unpack(dst)?;

    fs::remove_file(tar_path)?;
    Ok(())
}

fn deflate(bytes: bytes::Bytes, path: &Path) -> Result<()> {
    let mut gz = GzDecoder::new(&bytes[..]);
    let mut file = File::create(path)?;
    io::copy(&mut gz, &mut file)?;
    Ok(())
}

fn prepare_fs() -> Result<()> {
    let path = Path::new("/dev");
    if !path.exists() {
        fs::create_dir("/dev").context("create /dev")?;
        fs::set_permissions(path, Permissions::from_mode(RW_MODE))?;
    }
    let path = path.join("null");
    if !path.exists() {
        File::create("/dev/null")
            .context("create /dev/null")?
            .set_permissions(Permissions::from_mode(RW_MODE))
            .context("set /dev/null access mode")?;
    }
    Ok(())
}

fn change_root(path: &Path) -> Result<()> {
    unix::fs::chroot(path).context("chroot to tempDir")?;
    env::set_current_dir("/").context("set current dir to /")?;
    Ok(())
}

#[cfg(target_os = "linux")]
fn start_namespace() -> Result<()> {
    unsafe {
        libc::unshare(libc::CLONE_NEWPID);
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn start_namespace() -> Result<()> {
    Err(anyhow::anyhow!("Unsupported platform"))
}
