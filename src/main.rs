//! # Eezy Programming Language
//!
//! Eezy programming language is a programming language that aims to teach object oriented programming
//! with the abstraction provided by languages like Python and Go, excluding unnecessary verbosity and
//! enforcing better reachability among learners.

use std::path::PathBuf;

use clap::Parser;
use colored::Colorize;
use ezc::cli::{Cli, Commands};
use std::time::Instant;

fn main() {
    let cli = Cli::parse();
    let start = Instant::now();

    match cli.command {
        Commands::Run { source_path } => {
            // run the source code
            println!(
                "Run the source code at {}",
                source_path.iter().collect::<PathBuf>().display()
            );
        }
        Commands::Build { source_path } => {
            // build the source code
            println!(
                "Build the source code at {}",
                source_path.iter().collect::<PathBuf>().display()
            )
        }
    };

    let duration_ms = start.elapsed().as_millis();
    let duration_msg = if duration_ms < 120 {
        duration_ms.to_string().green()
    } else {
        duration_ms.to_string().red()
    };

    println!("Operation took {} ms", duration_msg);
}
