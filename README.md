# clang-sys-linkage

This crate is just a renamed `clang-sys` as of `v0.29.3` published as `v1.0.0`.

Cargo disallows more than one package in a dependency tree linking to the same
native library
[using the `package.links` Cargo manifest key](https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key).
Because `clang-sys` had not yet hit `1.0.0`, it was not uncommon for packages to
end up with multiple incompatible versions of `clang-sys` in their dependency
which resulted in Cargo refusing to compile the package. This crate exists to
allow the last few pre-`1.0.0` `clang-sys` versions to co-exist peacefully by
offloading the actual implementation of the crate to this crate. As such it is
obsoleted by `clang-sys` itself being bumped to `1.0.0` and is only used for
pre-`1.0.0` versions.
