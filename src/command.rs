use anyhow::{Context, Ok, Result};
use std::io::{self, Write};
use std::path::Path;
use std::process;

pub fn run(cmd: &Path, args: &[String]) -> Result<()> {
    assert!(cmd.exists(), "Command {cmd:?} does not exist.");
    assert!(Path::new("/dev/null").exists());

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
