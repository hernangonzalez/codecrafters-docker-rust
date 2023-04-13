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

    // todo: fs::prepare_root()
    hub::fetch(image)?;
    fs::isolate(command)?;
    command::run(command, command_args)
}
