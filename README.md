clang-sys
=========

[![crates.io](https://img.shields.io/crates/v/clang-sys.svg)](https://crates.io/crates/clang-sys)
[![Travis CI](https://travis-ci.org/KyleMayes/clang-sys.svg?branch=master)](https://travis-ci.org/KyleMayes/clang-sys)

Rust bindings for `libclang`.

Supported on the stable, beta, and nightly Rust channels.

Released under the Apache License 2.0.

### Dependencies

This crate depends on `libclang.so` (Linux), `libclang.dylib` (OS X), or `libclang.dll` (Windows).
These binaries can be either be installed as a part of Clang or downloaded
[here](http://llvm.org/releases/download.html).

The `libclang` binary will be searched for first by calling `llvm-config --libdir`. If this fails,
the `libclang` binary will be searched for in likely places (e.g., `/usr/local/lib/` on Linux). If
neither of these approaches is successful, you can specify the directory the `libclang` binary can
be found in with the `LIBCLANG_PATH` environment variable. The path to the `llvm-config` executable
you want to use can be specified with the `LLVM_CONFIG_PATH` environment variable.

If you want to link to `libclang` statically, enable the `static` feature. You can specify the
directory the various LLVM and Clang static libraries can be found in with the
`LIBCLANG_STATIC_PATH` environment variable. This feature is not supported for LLVM + Clang 3.8.

### Supported Versions

* 3.5.x - [Documentation](https://kylemayes.github.io/clang-sys/3_5/clang_sys)
* 3.6.x - [Documentation](https://kylemayes.github.io/clang-sys/3_6/clang_sys)
* 3.7.x - [Documentation](https://kylemayes.github.io/clang-sys/3_7/clang_sys)
* 3.8.x - [Documentation](https://kylemayes.github.io/clang-sys/3_8/clang_sys)

If you do not select a specific version, a common subset API will be availabile. The documentation
for this API is [here](https://kylemayes.github.io/clang-sys/all/clang_sys).
