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

**scheduler command**
```bash
cargo test --test arg_test -- scheduler  --help
```
output:
```bash
Rapidash scheduler Command Line Interface

Usage: arg_test-be8a5bf82f022b17 --name <NAME> scheduler [OPTIONS] <COMMAND>

Commands:
  start  Rapidash Scheduler Start
  stop   Rapidash Scheduler Stop
  help   Print this message or the help of the given subcommand(s)

Options:
  -l, --list     lists test values
  -h, --help     Print help information
  -V, --version  Print version information
```

**scheduler start command**
```bash
Rapidash Scheduler Start

Usage: arg_test-be8a5bf82f022b17 scheduler start

Options:
  -h, --help  Print help information
```

** Scheduler command**
```bash
cargo test --test arg_test -- scheduler -l  start
```

output:
```bash
argument Args { command: Scheduler { list: true, command: Start } }
```


## test main file
cargo run
```bash
cargo run -- scheduler --help
```
output:
```bash
Rapidash scheduler Command Line Interface

Usage: rapidash scheduler <COMMAND>

Commands:
  start  Start Service
  stop   Stop Service
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```