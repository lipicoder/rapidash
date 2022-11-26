//! Rapidash client

use error::Result;
use mimalloc::MiMalloc;

// use mimalloc replace rust default alloc
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use clap::Parser;
use rapidash::cli::Args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    println!("argument {:?}", args);

    let level_one = args.command;
    println!("level one {:?}", level_one);

    Ok(())
}

#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        println!("Hello, world!");
        assert_eq!(2 + 2, 4);
    }
}
