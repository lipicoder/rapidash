use ballista::prelude::*;
use datafusion::prelude::{col, lit, ParquetReadOptions};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();
    let config = BallistaConfig::builder()
        .set("ballista.shuffle.partitions", "4")
        .build()?;

    // connect to Ballista scheduler
    let ctx = BallistaContext::remote("localhost", 50050, &config).await?;
    println!("connect server:{}", start.elapsed().as_secs_f64());

    let filepath =
        "/Users/lipi/dev/reckon/tests/mock/stocks/0d0c43f3e67b43689c8001e8bbbe16ca.parquet";

    // define the query using the DataFrame trait
    let df = ctx
        .read_parquet(filepath, ParquetReadOptions::default())
        .await?;
    println!("read parquet:{}", start.elapsed().as_secs_f64());
    println!("df:{:?}", df);

    // let result = df
    //     .select_columns(&["sec_code", "date", "close"])?
    //     .filter(col("sec_code").eq(lit("600856")))?;

    let result = df
        .select_columns(&["sec_code", "date", "close"])?
        .filter(col("close").gt(lit(89)))?;

    println!("df after select:{:?}", result);

    println!("select and filter:{}", start.elapsed().as_secs_f64());

    // print the results
    result.show().await?;

    println!("show:{}", start.elapsed().as_secs_f64());

    Ok(())
}
