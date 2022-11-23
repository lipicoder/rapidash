//! Argument struct for the CLI.
// use std::env;
use std::path::Path;

use clap::Parser;

#[derive(Debug, Parser, PartialEq)]
#[command(author, version, about, long_about= None)]
struct Args {
    #[arg(
        short = 'p',
        long,
        help = "Path to your data, default to current directory",
        value_parser = is_valid_data_dir
    )]
    data_path: Option<String>,

    #[clap(
        short = 'c',
        long,
        help = "The batch size of each query, or use Ballista default",
        value_parser = is_valid_batch_size
    )]
    batch_size: Option<usize>,

    #[clap(
        long,
        help = "The max concurrent tasks, only for Ballista local mode. Default: all available cores",
        value_parser = is_valid_concurrent_tasks_size
    )]
    concurrent_tasks: Option<usize>,

    #[clap(long, help = "Rapidash scheduler host")]
    host: Option<String>,

    #[clap(long, help = "Rapidash scheduler port")]
    port: Option<u16>,

    #[clap(
        short,
        long,
        help = "Reduce printing other than the results and work quietly"
    )]
    quiet: bool,
}

#[allow(dead_code)]
fn is_valid_file(dir: &str) -> std::result::Result<(), String> {
    if Path::new(dir).is_file() {
        Ok(())
    } else {
        Err(format!("Invalid file '{}'", dir))
    }
}

fn is_valid_data_dir(dir: &str) -> std::result::Result<(), String> {
    if Path::new(dir).is_dir() {
        Ok(())
    } else {
        Err(format!("Invalid data directory '{}'", dir))
    }
}

fn is_valid_batch_size(size: &str) -> std::result::Result<(), String> {
    match size.parse::<usize>() {
        Ok(size) if size > 0 => Ok(()),
        _ => Err(format!("Invalid batch size '{}'", size)),
    }
}

fn is_valid_concurrent_tasks_size(size: &str) -> std::result::Result<(), String> {
    match size.parse::<usize>() {
        Ok(size) if size > 0 => Ok(()),
        _ => Err(format!("Invalid concurrent_tasks size '{}'", size)),
    }
}
