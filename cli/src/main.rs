//! Rapidash client

use error::Result;
use mimalloc::MiMalloc;

// use mimalloc replace rust default alloc
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}
