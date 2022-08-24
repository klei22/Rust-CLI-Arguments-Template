# Rust Template for CLI Arguments


This is a plain template for how to utilize cli arguments in Rust.

## Current Examples

- [x] options (e.g. "-f file.txt")
- [x] true/false flags (e.g. "-q" for "quiet")
- [ ] options with matching

## Workflow

- `cargo build` - build
- `cargo clippy` - run the friendly rust linter
- `cargo run` - run
- `cargo run -q` - run quiet
- `cargo run -q -- -f just_a_test.txt` - run with file input flag
- `cargo run -q -- -f just_a_test.txt -A` - run with boolean flag

## Documentation

- `cargo doc` - create documentation
- `cargo doc --open` - open documentation

## References

- [clap-rs-clap-rust-command-line](https://rustrepo.com/repo/clap-rs-clap-rust-command-line#using-builder-pattern)
