clang-sys
=========

Rust bindings for libclang.

Released under the MIT license.

### Dependencies

This crate depends on `libclang.dll` (Windows), `libclang.so` (Linux), or `libclang.dylib` (OS X).
These binaries can be either be installed as a part of clang or downloaded
[here](http://llvm.org/releases/download.html).

#### Windows

On Windows, `libclang.dll` should be placed in `<rust>\lib\rustlib\*-pc-windows-*\lib` where
`<rust>` is your Rust installation directory.
