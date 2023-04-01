use anyhow::{Context, Result};
use std::fs;
use std::io::{self, Write};
use std::os::unix;
use std::path::Path;
use tempfile::{tempdir, TempDir};

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
    unix::fs::chroot(root).context("could not change root")?;

    let output = std::process::Command::new(command)
        .args(command_args)
        .output()?;

    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    if let Some(code) = output.status.code() {
        std::process::exit(code);
    }

    Ok(())
}

fn prepare_root(cmd: &Path) -> Result<TempDir> {
    let filename = cmd.file_name().context("cmd filename")?;
    let temp_dir = tempdir()?;

    let mut path = temp_dir.path().join(cmd.strip_prefix("/")?);
    path.pop();
    fs::create_dir(path.as_path())?;

    path.push(filename);
    fs::copy(cmd, path)?;

    Ok(temp_dir)
}
