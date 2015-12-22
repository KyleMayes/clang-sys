clang-sys
=========

[![crates.io](https://img.shields.io/crates/v/clang-sys.svg)](https://crates.io/crates/clang-sys)

Rust bindings for libclang.

Released under the MIT license.

### Dependencies

This crate depends on `libclang.dll` (Windows), `libclang.so` (Linux), or `libclang.dylib` (OS X).
These binaries can be either be installed as a part of clang or downloaded
[here](http://llvm.org/releases/download.html).

#### Windows

On Windows, `libclang.dll` should be placed in `<rust>\lib\rustlib\*-pc-windows-*\lib` where
`<rust>` is your Rust installation directory.

### Supported Versions

* 3.5.x - [Documentation](https://kylemayes.github.io/clang-sys/3_5/clang_sys)
* 3.6.x - [Documentation](https://kylemayes.github.io/clang-sys/3_6/clang_sys)
* 3.7.x - [Documentation](https://kylemayes.github.io/clang-sys/3_7/clang_sys)

If you do not select a specific version, a common subset API will be availabile. The documentation
for this API is [here](https://kylemayes.github.io/clang-sys/all/clang_sys).
