use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::fs::File;
use std::fs::Permissions;
use std::io::{self, Write};
use std::os::unix;
use std::path::Path;
use std::process;
use tempfile::{tempdir, TempDir};
use unix::fs::PermissionsExt;

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 4 {
        return Err(anyhow::anyhow!("Not enough arguments"));
    }

    let _action = &args[1];
    let _image = &args[2];
    let command = Path::new(&args[3]);
    let command_args = &args[4..];

    let root = prepare_root(command)?;
    unix::fs::chroot(root)?;
    env::set_current_dir("/")?;

    fs::create_dir("/dev").context("create /dev")?;
    File::create("/dev/null")?.set_permissions(Permissions::from_mode(0o666))?;

    let child_process = process::Command::new(command)
        .args(command_args)
        .spawn()
        .with_context(|| {
            format!(
                "Tried to run '{:?}' with arguments {:?}",
                command, command_args
            )
        })?;

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

fn prepare_root(cmd: &Path) -> Result<TempDir> {
    let access_mode = 0o777;

    let filename = cmd.file_name().context("cmd filename")?;
    let temp_dir = tempdir()?;

    let mut path = temp_dir.path().join(cmd.strip_prefix("/")?);
    path.pop();
    fs::create_dir(path.as_path())?;

    path.push(filename);
    fs::copy(cmd, path.as_path())?;

    let perm = Permissions::from_mode(access_mode);
    fs::set_permissions(path, perm)?;

    Ok(temp_dir)
}
