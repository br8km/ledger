#![allow(dead_code)]

use anyhow::{Context, Result};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn answer() -> i32 {
  42
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}


fn main() -> Result<()> {
    println!("This is information");
    eprintln!("This is an error! :(");
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
