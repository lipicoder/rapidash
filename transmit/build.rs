//! build proto to rust
fn main() -> Result<(), String> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/generated")
        .compile(&["src/proto/rapidash.proto"], &["proto"])
        .unwrap();
    Ok(())
}
