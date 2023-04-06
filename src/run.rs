use anyhow::{Context, Ok, Result};
use std::env;
use std::fs::{self, File, Permissions};
use std::io::{self, Write};
use std::os::unix::{self, fs::PermissionsExt};
use std::path::Path;
use std::process;
use tempfile::tempdir;

const EXEC_MODE: u32 = 0o777;
const RW_MODE: u32 = 0o666;

pub fn command(cmd: &Path, args: &[String]) -> Result<()> {
    prepare_root(cmd)?;
    prepare_fs()?;
    start_namespace()?;

    let child_process = process::Command::new(cmd)
        .args(args)
        .spawn()
        .with_context(|| format!("Tried to run '{:?}' with arguments {:?}", cmd, args))?;

    let output = child_process
        .wait_with_output()
        .expect("Failed to wait on child");

    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    if let Some(code) = output.status.code() {
        std::process::exit(code);
    }

    Ok(())
}

fn prepare_fs() -> Result<()> {
    fs::create_dir("/dev").context("create /dev")?;
    fs::set_permissions("/dev", Permissions::from_mode(RW_MODE))?;
    File::create("/dev/null")
        .context("create /dev/null")?
        .set_permissions(Permissions::from_mode(RW_MODE))
        .context("set /dev/null access mode")?;
    Ok(())
}

fn prepare_root(cmd: &Path) -> Result<()> {
    let filename = cmd.file_name().context("cmd filename")?;
    let temp_dir = tempdir()?;

    let mut path = temp_dir.path().join(cmd.strip_prefix("/")?);
    path.pop();
    fs::create_dir_all(&path).context("create dir for command")?;

    path.push(filename);
    fs::copy(cmd, path.as_path())?;

    let perm = Permissions::from_mode(EXEC_MODE);
    fs::set_permissions(path, perm)?;

    unix::fs::chroot(temp_dir.path()).context("chroot to tempDir")?;
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
