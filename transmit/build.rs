//! build proto to rust
fn main() -> Result<(), String> {
    tonic_build::configure()
        .build_server(false)
        // .out_dir("proto")
        .compile(&["proto/rapidash.proto"], &["proto"])
        .unwrap();
    Ok(())
}
