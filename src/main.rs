mod command;
mod fs;
mod hub;

use anyhow::{self, ensure, Result};
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    ensure!(args.len() >= 4, "Not enough arguments");

    let action = &args[1];
    let image = &args[2];
    let command = Path::new(&args[3]);
    let command_args = &args[4..];
    ensure!(action == "run", "Only `run` is a supported action");

    let path = fs::create_root()?;
    hub::pull(image, &path)?;
    fs::isolate(&path)?;
    command::run(command, command_args)
}
