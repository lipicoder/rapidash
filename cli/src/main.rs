//! Rapidash client

use error::Result;
use mimalloc::MiMalloc;

// use mimalloc replace rust default alloc
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use clap::Parser;
use rapidash::cli::{Args, Operator, Stage};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // check scheduler service

    match args.command {
        Stage::Scheduler { command } => {
            println!("scheduler command :{:?}", command);
            match command {
                Operator::Start { .. } => {
                    println!("start scheduler");
                }
                Operator::Stop { .. } => {
                    println!("stop scheduler");
                }
            }
        }
        Stage::Executor { command } => {
            println!("executor command :{:?}", command);
            match command {
                Operator::Start { .. } => {
                    println!("start executor");
                }
                Operator::Stop { .. } => {
                    println!("stop executor");
                }
            }
        }
    }

    Ok(())
}
