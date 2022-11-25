//! Clap arugment tests
use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: LevelOne,

    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

/// Level one command.
#[derive(Subcommand, Debug)]
enum LevelOne {
    /// scheduler
    #[command(author, version, about = "Rapidash scheduler Command Line Interface")]
    Scheduler {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },

    /// Executor
    Executor {},
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
