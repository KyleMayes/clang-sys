clang-sys
=========

[![crates.io](https://img.shields.io/crates/v/clang-sys.svg)](https://crates.io/crates/clang-sys)

Rust bindings for `libclang`.

Released under the Apache License 2.0

### Dependencies

This crate depends on `libclang.dll` (Windows), `libclang.so` (Linux), or `libclang.dylib` (OS X).
These binaries can be either be installed as a part of Clang or downloaded
[here](http://llvm.org/releases/download.html).

The `libclang` binary will be looked for in likely places (e.g., `/usr/lib` on Linux), but you can
specify the directory the `libclang` binary is in with the `LIBCLANG_PATH` environment variable.

If you want to link to `libclang` statically, set the `LIBCLANG_STATIC` environment variable or
enable the `static` feature. You can specify the directory the various LLVM and Clang static
libraries are searched for with the `LIBCLANG_STATIC_PATH` environment variable.

### Supported Versions

* 3.4.x - [Documentation](https://kylemayes.github.io/clang-sys/3_4/clang_sys)
* 3.5.x - [Documentation](https://kylemayes.github.io/clang-sys/3_5/clang_sys)
* 3.6.x - [Documentation](https://kylemayes.github.io/clang-sys/3_6/clang_sys)
* 3.7.x - [Documentation](https://kylemayes.github.io/clang-sys/3_7/clang_sys)
* 3.8.x - [Documentation](https://kylemayes.github.io/clang-sys/3_8/clang_sys)

If you do not select a specific version, a common subset API will be availabile. The documentation
for this API is [here](https://kylemayes.github.io/clang-sys/all/clang_sys).
