//! Argument struct for the CLI.
// use std::env;
use crate::validator::{is_valid_batch_size, is_valid_data_dir};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser, PartialEq)]
#[command(author, version, about, long_about= None)]
pub struct Args {
    #[command(subcommand)]
    pub command: LevelOne,

    #[arg(
        short = 'p',
        long,
        help = "Path to your data, default to current directory",
        value_parser = is_valid_data_dir,
    )]
    data_path: Option<String>,

    #[arg(
        short = 'c',
        long,
        help = "The batch size of each query, or use Rapidash default",
        value_parser = is_valid_batch_size,
    )]
    batch_size: Option<usize>,

    #[arg(long, help = "Rapidash scheduler host", default_value = "127.0.0.1")]
    host: Option<String>,

    #[arg(long, help = "Rapidash scheduler port", default_value = "51008")]
    port: Option<u16>,
}

/// Level one command.
#[derive(Subcommand, PartialEq, Debug)]
pub enum LevelOne {
    /// scheduler
    #[command(author, version, about = "Rapidash scheduler Command Line Interface")]
    Scheduler {
        #[command(subcommand)]
        command: LevelTwo,
    },

    /// Executor
    Executor {
        #[command(subcommand)]
        command: LevelTwo,
    },
}

#[derive(Subcommand, PartialEq, Debug)]
pub enum LevelTwo {
    /// test start subcommand
    #[command(about = "Start Service")]
    Start {},

    /// test stop subcommand
    #[command(about = "Stop Service")]
    Stop {},
}
