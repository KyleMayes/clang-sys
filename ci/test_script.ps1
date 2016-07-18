rustc --version
cargo --version

$env:RUST_BACKTRACE = 1
cargo test --verbose --features ${env:CLANG_VERSION} -- --nocapture
