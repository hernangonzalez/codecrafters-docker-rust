use anyhow::Result;
use std::io::{self, Write};

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() -> Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Spining up mydocker ...");

    let args: Vec<_> = std::env::args().collect();
    if args.len() < 4 {
        return Err(anyhow::anyhow!("Not enough arguments"));
    }

    let command = &args[3];
    let command_args = &args[4..];
    let output = std::process::Command::new(command)
        .args(command_args)
        .output()?;

    println!("{}", output.status);
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    Ok(())
}
