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

#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        println!("Hello, world!");
        assert_eq!(2 + 2, 4);
    }
}
