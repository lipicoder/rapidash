//! Argument struct for the CLI.
// use std::env;
use crate::validator::{is_valid_batch_size, is_valid_data_dir};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about= None)]
struct Cli {
    #[command(subcommand)]
    command: LevelOne,

    #[arg(
        short = 'p',
        long,
        help = "Path to your data, default to current directory",
        value_parser = is_valid_data_dir
    )]
    data_path: Option<String>,

    #[arg(
        short = 'c',
        long,
        help = "The batch size of each query, or use Rapidash default",
        value_parser = is_valid_batch_size
    )]
    batch_size: Option<usize>,

    #[arg(long, help = "Rapidash scheduler host")]
    host: Option<String>,

    #[arg(long, help = "Rapidash scheduler port")]
    port: Option<u16>,
}

/// Level one command.
#[derive(Subcommand)]
enum LevelOne {
    /// scheduler
    Scheduler {},

    /// Executor
    Executor {},
}

#[cfg(test)]

mod tests {
    use clap::Parser;
    use error::Result;
    use log::{debug, info};

    /// Simple program to greet a person
    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    struct Args {
        /// Name of the person to greet
        #[arg(short, long)]
        name: String,

        /// Number of times to greet
        #[arg(short, long, default_value_t = 1)]
        count: u8,
    }

    #[test]
    fn arg_command() -> Result<()> {
        env_logger::init();
        let args = Args::parse();
        debug!("debug args {:?}", args);
        info!("info args {:?}", args);

        for _ in 0..args.count {
            println!("Hello {}!", args.name)
        }

        Ok(())
    }
}
