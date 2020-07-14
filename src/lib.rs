// Copyright 2016 Kyle Mayes
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Rust bindings for `libclang`.
//!
//! ## Documentation
//!
//! There are two versions of the documentation, one for the API exposed when
//! linking dynamically or statically and one for the API exposed when linking
//! at runtime (see the
//! [Dependencies](https://github.com/KyleMayes/clang-sys#dependencies) section
//! of the README for more information on the linking options).
//!
//! The only difference between the APIs exposed is that when linking at runtime
//! a few additional types and functions are exposed to manage the loaded
//! `libclang` shared library.
//!
//! * Runtime - [Documentation](https://kylemayes.github.io/clang-sys/runtime/clang_sys)
//! * Dynamic / Static - [Documentation](https://kylemayes.github.io/clang-sys/default/clang_sys)

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]

extern crate clang_sys_linkage;

pub use clang_sys_linkage::*;
