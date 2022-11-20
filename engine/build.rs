//! build script

fn main() -> Result<(), String> {
    use std::io::Write;

    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // for use in docker build where file changes can be wonky
    println!("cargo:rerun-if-env-changed=FORCE_REBUILD");

    println!("cargo:rerun-if-changed=proto/rapidash.proto");
    let version = rustc_version::version().unwrap();
    println!("cargo:rustc-env=RUSTC_VERSION={}", version);
    println!("cargo:rerun-if-changed=proto/datafusion.proto");
    tonic_build::configure()
        .extern_path(".datafusion", "::datafusion_proto::protobuf")
        .compile(&["proto/rapidash.proto"], &["proto"])
        .map_err(|e| format!("protobuf compilation failed: {}", e))?;

    // TODO: undo when resolved: https://github.com/intellij-rust/intellij-rust/issues/9402
    #[cfg(feature = "docsrs")]
    let path = out.join("rapidash.rs");
    #[cfg(not(feature = "docsrs"))]
    let path = "src/generated/rapidash.rs";

    let code = std::fs::read_to_string(out.join("rapidash.protobuf.rs")).unwrap();
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .unwrap();
    file.write_all(code.as_str().as_ref()).unwrap();

    Ok(())
}
