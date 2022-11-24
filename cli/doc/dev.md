//! Development record.

## test special test file

**Cargo.toml**
```toml
[[test]]
name = "arg_test"
path = "tests/args.rs"
harness = false
```


```bash
cargo test --test arg_test -- --name hello
```