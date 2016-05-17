clang-sys
=========

[![crates.io](https://img.shields.io/crates/v/clang-sys.svg)](https://crates.io/crates/clang-sys)
[![Travis CI](https://travis-ci.org/KyleMayes/clang-sys.svg?branch=master)](https://travis-ci.org/KyleMayes/clang-sys)

Rust bindings for `libclang`.

Supported on the stable, beta, and nightly Rust channels.

Released under the Apache License 2.0.

### Supported Versions

To target a version of `libclang`, enable one of the following Cargo features:

* `clang_3_5` - requires `libclang` 3.5 or later
  ([Documentation](https://kylemayes.github.io/clang-sys/3_5/clang_sys))
* `clang_3_6` - requires `libclang` 3.6 or later
  ([Documentation](https://kylemayes.github.io/clang-sys/3_6/clang_sys))
* `clang_3_7` - requires `libclang` 3.7 or later
  ([Documentation](https://kylemayes.github.io/clang-sys/3_7/clang_sys))
* `clang_3_8` - requires `libclang` 3.8 or later
  ([Documentation](https://kylemayes.github.io/clang-sys/3_8/clang_sys))

If you do not enable one of these features, the API provided by `libclang` 3.5 will be available by
default.

### Dependencies

By default, this crate will attempt to link to `libclang` dynamically. In this case, this crate
depends on the `libclang` shared library (`libclang.so` on Linux, `libclang.dylib` on OS X,
`libclang.dll` on Windows). If you want to link to `libclang` statically instead, enable the
`static` Cargo feature. In this case, this crate depends on the LLVM and Clang static libraries.

These libraries can be either be installed as a part of Clang or downloaded
[here](http://llvm.org/releases/download.html).

**Note:** The downloads for LLVM and Clang 3.8 do not include the `libclang.a` static library. This
means you cannot link to this version of `libclang` statically unless you build it from source.

#### Environment Variables

The following environment variables, if set, are used by this crate to find the required libraries:

* `LLVM_CONFIG_PATH` - provides a path to an `llvm-config` executable
* `LIBCLANG_PATH` - provides a path to a directory containing a `libclang` shared library
* `LIBCLANG_STATIC_PATH` - provides a path to a directory containing LLVM and Clang static libraries

#### Dynamic

First, the `libclang` shared library will be searched for in the directory provided by the
`LIBCLANG_PATH` environment variable if it was set. If this fails, the directory returned by
`llvm-config --libdir` will be searched. If neither of these approaches is successful, a list of
likely directories will be searched (e.g., `/usr/local/lib` on Linux).

#### Static

The availability of `llvm-config` is not optional for static linking. Ensure that an instance of
this executable can be found on your system's path or set the `LLVM_CONFIG_PATH` environment
variable. The required LLVM and Clang static libraries will be searched for in the same way as the
shared library is searched for, except the `LIBCLANG_STATIC_PATH` environment variable is used in
place of the `LIBCLANG_PATH` environment variable.
