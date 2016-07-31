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

//! Finds and links to the required `libclang` libraries.

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", warn(clippy))]

extern crate glob;

use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command};

use glob::{MatchOptions};

// Environment variables:
//
// * LLVM_CONFIG_PATH - provides a path to an `llvm-config` executable
// * LIBCLANG_PATH - provides a path to a directory containing a `libclang` shared library
// * LIBCLANG_STATIC_PATH - provides a path to a directory containing LLVM and Clang static libraries

/// Returns whether the supplied directory contains the supplied file.
fn contains<D: AsRef<Path>>(directory: D, file: &str) -> bool {
    directory.as_ref().join(file).exists()
}

/// Runs a console command, returning the output if the command was successfully executed.
fn run(command: &str, arguments: &[&str]) -> Option<String> {
    Command::new(command).args(arguments).output().map(|o| {
        String::from_utf8_lossy(&o.stdout).into_owned()
    }).ok()
}

/// Runs `llvm-config`, returning the output if the command was successfully executed.
fn run_llvm_config(arguments: &[&str]) -> Option<String> {
    run(&env::var("LLVM_CONFIG_PATH").unwrap_or("llvm-config".into()), arguments)
}

/// Backup search directory globs for FreeBSD and Linux.
const SEARCH_LINUX: &'static [&'static str] = &[
    "/usr/lib*",
    "/usr/lib*/*",
    "/usr/lib*/*/*",
    "/usr/local/lib*",
    "/usr/local/lib*/*",
    "/usr/local/lib*/*/*",
    "/usr/local/llvm*/lib",
];

/// Backup search directory globs for OS X.
const SEARCH_OSX: &'static [&'static str] = &[
    "/usr/local/opt/llvm*/lib",
    "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib",
    "/Library/Developer/CommandLineTools/usr/lib",
    "/usr/local/opt/llvm*/lib/llvm*/lib",
];

/// Backup search directory globs for Windows.
const SEARCH_WINDOWS: &'static [&'static str] = &[
    "C:\\LLVM\\bin",
    "C:\\LLVM\\lib",
    "C:\\Program Files*\\LLVM\\bin",
    "C:\\Program Files*\\LLVM\\lib",
];

/// Searches for a library, returning the directory it can be found in if the search was successful.
fn find(file: &str, env: &str) -> Result<PathBuf, String> {
    // Search the directory provided by the relevant environment variable, if set.
    if let Some(directory) = env::var(env).map(|d| Path::new(&d).to_path_buf()).ok() {
        if contains(&directory, file) {
            return Ok(directory);
        }

        // On Windows, `libclang.dll` is usually found in the LLVM `bin` directory while
        // `libclang.lib` is usually found in the LLVM `lib` directory. Search the other if one is
        // specified with `LIBCLANG_PATH`.
        if cfg!(target_os="windows") {
            let suffix = if directory.ends_with("lib") {
                Some("bin")
            } else if directory.ends_with("bin") {
                Some("lib")
            } else {
                None
            };
            if let Some(suffix) = suffix {
                let alternative = directory.parent().unwrap().join(suffix);
                if contains(&alternative, file) {
                    return Ok(alternative);
                }
            }
        }
    }

    // Search the `bin` and `lib` subdirectories in the directory returned by
    // `llvm-config --prefix`, if `llvm-config` is available.
    if let Some(output) = run_llvm_config(&["--prefix"]) {
        let directory = Path::new(output.lines().next().unwrap()).to_path_buf();
        let bin = directory.join("bin");
        if contains(&bin, file) {
            return Ok(bin);
        }
        let lib = directory.join("lib");
        if contains(&lib, file) {
            return Ok(lib);
        }
    }

    // Search the backup directories.
    let search = if cfg!(any(target_os="freebsd", target_os="linux")) {
        SEARCH_LINUX
    } else if cfg!(target_os="macos") {
        SEARCH_OSX
    } else if cfg!(target_os="windows") {
        SEARCH_WINDOWS
    } else {
        &[]
    };
    for pattern in search {
        let mut options = MatchOptions::new();
        options.case_sensitive = false;
        options.require_literal_separator = true;
        if let Ok(paths) = glob::glob_with(pattern, &options) {
            for path in paths.filter_map(Result::ok).filter(|p| p.is_dir()) {
                if contains(&path, file) {
                    return Ok(path);
                }
            }
        }
    }
    let message = format!(
        "couldn't find '{0}', set the {1} environment variable to a path where '{0}' can be found",
        file,
        env,
    );
    Err(message)
}

/// Returns the name of an LLVM or Clang library from a path.
fn get_library_name(path: &Path) -> Option<String> {
    path.file_stem().map(|l| l.to_string_lossy()[3..].into())
}

/// Returns the LLVM libraries required to link to `libclang` statically.
fn get_llvm_libraries() -> Vec<String> {
    run_llvm_config(&["--libs"]).expect(
        "couldn't execute `llvm-config --libs`, set the LLVM_CONFIG_PATH environment variable to \
         a path to an `llvm-config` executable"
    ).split_whitespace().filter_map(|p| {
        // Depending on the version of `llvm-config` in use, listed libraries may be in one of two
        // forms, a full path to the library or simply prefixed with `-l`.
        if p.starts_with("-l") {
            Some(p[2..].into())
        } else {
            get_library_name(Path::new(p))
        }
    }).collect()
}

/// Clang libraries required to link to `libclang` 3.5 and later statically.
const CLANG_LIBRARIES: &'static [&'static str] = &[
    "clang",
    "clangAST",
    "clangAnalysis",
    "clangBasic",
    "clangDriver",
    "clangEdit",
    "clangFrontend",
    "clangIndex",
    "clangLex",
    "clangParse",
    "clangRewrite",
    "clangSema",
    "clangSerialization",
];

/// Returns the Clang libraries required to link to `libclang` statically.
fn get_clang_libraries<P: AsRef<Path>>(directory: P) -> Vec<String> {
    let pattern = directory.as_ref().join("libclang*.a").to_string_lossy().to_string();
    if let Ok(libraries) = glob::glob(&pattern) {
        libraries.filter_map(|l| l.ok().and_then(|l| get_library_name(&l))).collect()
    } else {
        CLANG_LIBRARIES.iter().map(|l| l.to_string()).collect()
    }
}

fn main() {
    if cfg!(feature="static") {
        // Find LLVM and Clang static libraries.
        let directory = find("libclang.a", "LIBCLANG_STATIC_PATH").unwrap();

        print!("cargo:rustc-flags=");

        // Specify required LLVM and Clang static libraries.
        print!("-L {} ", directory.display());
        for library in get_llvm_libraries() {
            print!("-l static={} ", library);
        }
        for library in get_clang_libraries(&directory) {
            print!("-l static={} ", library);
        }

        // Specify required system libraries.
        if cfg!(any(target_os="freebsd", target_os="linux")) {
            println!("-l ffi -l ncursesw -l stdc++ -l z");
        } else if cfg!(target_os="macos") {
            println!("-l ffi -l ncurses -l stdc++ -l z");
        } else {
            panic!("unsupported operating system for static linking");
        }
    } else {
        let file = if cfg!(target_os="windows") {
            // The filename of the `libclang` shared library on Windows is `libclang.dll` instead of
            // the expected `clang.dll`.
            "libclang.dll".into()
        } else {
            format!("{}clang{}", env::consts::DLL_PREFIX, env::consts::DLL_SUFFIX)
        };

        // Find the `libclang` shared library.
        let directory = find(&file, "LIBCLANG_PATH").unwrap();

        println!("cargo:rustc-link-search={}", directory.display());
        if cfg!(all(target_os="windows", target_env="msvc")) {
            // Find the `libclang` stub static library required for the MSVC toolchain.
            let directory = find("libclang.lib", "LIBCLANG_PATH").unwrap();

            println!("cargo:rustc-link-search={}", directory.display());
            println!("cargo:rustc-link-lib=dylib=libclang");
        } else {
            println!("cargo:rustc-link-lib=dylib=clang");
        }
    }
}
