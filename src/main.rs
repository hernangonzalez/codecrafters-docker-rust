mod command;

use anyhow::{self, Result};
use std::path::Path;

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

    command::run(command, command_args)
}
