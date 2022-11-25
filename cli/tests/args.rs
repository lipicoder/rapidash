//! Clap arugment tests
use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: LevelOne,
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

        /// test subcommand
        #[command(subcommand)]
        command: LevelTwo,
    },

    /// Executor
    Executor {},
}

#[derive(Subcommand, Debug)]
enum LevelTwo {
    /// test start subcommand
    #[command(about = "Rapidash Scheduler Start")]
    Start {},

    /// test stop subcommand
    #[command(about = "Rapidash Scheduler Stop")]
    Stop {},
}

fn main() {
    let args = Args::parse();

    println!("argument {:?}", args);
}
