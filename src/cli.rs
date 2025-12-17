use clap::{Parser, Subcommand, ValueHint};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "ezc",
    about = "Compiler toolchain for the Eezy programming language"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run an Eezy source file
    #[command(
        name = "run",
        about = "runs eezy source code and prints output directly to standard output",
        arg_required_else_help = true
    )]
    Run {
        /// Path(s) to the Eezy source file(s)
        #[arg(value_name = "SOURCE_FILE", value_hint = ValueHint::FilePath, required = true, num_args = 1)]
        source_path: Vec<PathBuf>,
    },

    /// Build an Eezy source file
    #[command(
        name = "build",
        about = "build eezy source code into JVM native byte code",
        arg_required_else_help = true
    )]
    Build {
        /// Path(s) to the Eezy source file(s)
        #[arg(value_name = "SOURCE_FILE", value_hint = ValueHint::FilePath, required = true, num_args = 1)]
        source_path: Vec<PathBuf>,
    },
}
