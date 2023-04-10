mod command;
mod fs;
mod image;

use anyhow::{self, Result};
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 4 {
        return Err(anyhow::anyhow!("Not enough arguments"));
    }

    let _action = &args[1];
    let _image = &args[2];
    let command = Path::new(&args[3]);
    let command_args = &args[4..];

    // image::fetch(image)?;
    fs::isolate(command)?;
    command::run(command, command_args)
}
