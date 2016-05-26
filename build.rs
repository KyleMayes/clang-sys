extern crate glob;

use std::env;
use std::path::{Path};
use std::process::{Command};

// Environment variables:
//
// * LLVM_CONFIG_PATH - provides a path to an `llvm-config` executable
// * LIBCLANG_PATH - provides a path to a directory containing a `libclang` shared library
// * LIBCLANG_STATIC_PATH - provides a path to a directory containing LLVM and Clang static libraries

/// Returns whether the supplied directory contains the supplied file.
fn contains(directory: &str, file: &str) -> bool {
    Path::new(&directory).join(&file).exists()
}

/// Panics with a user friendly error message.
fn error(file: &str, env: &str) -> ! {
    panic!("could not find {0}, set the {1} environment variable to a path where {0} can be found", file, env);
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

/// Backup search directories for Linux.
const SEARCH_LINUX: &'static [&'static str] = &[
    "/usr/lib",
    "/usr/lib/llvm",
    "/usr/lib/llvm-3.8/lib",
    "/usr/lib/llvm-3.7/lib",
    "/usr/lib/llvm-3.6/lib",
    "/usr/lib/llvm-3.5/lib",
    "/usr/lib/llvm-3.4/lib",
    "/usr/local/llvm38/lib",
    "/usr/local/llvm37/lib",
    "/usr/local/llvm36/lib",
    "/usr/local/llvm35/lib",
    "/usr/local/llvm34/lib",
    "/usr/lib64/llvm",
    "/usr/lib/x86_64-linux-gnu",
    "/usr/local/lib",
];

/// Backup search directories for OS X.
const SEARCH_OSX: &'static [&'static str] = &[
    "/usr/local/opt/llvm/lib",
    "/Library/Developer/CommandLineTools/usr/lib",
    "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib",
    "/usr/local/opt/llvm35/lib/llvm-3.8/lib",
    "/usr/local/opt/llvm35/lib/llvm-3.7/lib",
    "/usr/local/opt/llvm35/lib/llvm-3.6/lib",
    "/usr/local/opt/llvm35/lib/llvm-3.5/lib",
];

/// Backup search directories for Windows.
const SEARCH_WINDOWS: &'static [&'static str] = &[
    "C:\\Program Files\\LLVM\\bin",
    "C:\\Program Files\\LLVM\\lib",
];

/// Searches for a library, returning the directory it can be found in if the search was successful.
fn find(file: &str, env: &str) -> Option<String> {
    // Search the directory provided by the relevant environment variable, if set.
    if let Some(directory) = env::var(env).ok() {
        if contains(&directory, file) {
            return Some(directory);
        }
    }

    // Search the directory returned by `llvm-config --libdir`, if `llvm-config` is available.
    if let Some(output) = run_llvm_config(&["--libdir"]) {
        let directory = output.lines().map(|s| s.to_string()).next().unwrap();
        if contains(&directory, file) {
            return Some(directory);
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
        return None;
    };
    search.iter().find(|d| contains(d, file)).map(|s| s.to_string())
}

/// Returns the name of an LLVM or Clang library from a path.
fn get_library(path: &Path) -> Option<String> {
    path.file_stem().map(|l| l.to_string_lossy()[3..].into())
}

/// Returns the LLVM libraries required to link to `libclang` statically.
fn get_llvm_libraries() -> Vec<String> {
    run_llvm_config(&["--libs"]).expect(
        "could not execute `llvm-config --libs`, set the LLVM_CONFIG_PATH environment variable to \
         a path to an `llvm-config` executable"
    ).split_whitespace().filter_map(|p| {
        // Depending on the version of `llvm-config` in use, listed libraries may be in one of two
        // forms, a full path to the library or simply prefixed with `-l`.
        if p.starts_with("-l") {
            Some(p[2..].into())
        } else {
            get_library(&Path::new(p))
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
fn get_clang_libraries(directory: &str) -> Vec<String> {
    let pattern = Path::new(directory).join("libclang*.a").to_string_lossy().to_string();
    if let Ok(libraries) = glob::glob(&pattern) {
        libraries.filter_map(|l| l.ok().and_then(|l| get_library(&l))).collect()
    } else {
        CLANG_LIBRARIES.iter().map(|l| l.to_string()).collect()
    }
}

fn main() {
    if cfg!(feature="static") {
        // Find LLVM and Clang static libraries.
        let directory = match find("libclang.a", "LIBCLANG_STATIC_PATH") {
            Some(directory) => directory,
            _ => error("libclang.a", "LIBCLANG_STATIC_PATH"),
        };

        print!("cargo:rustc-flags=");

        // Specify required LLVM and Clang static libraries.
        print!("-L {} ", directory);
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
        };
    } else {
        let file = if cfg!(target_os="windows") {
            // The filename of the `libclang` shared library on Windows is `libclang.dll` instead of
            // the expected `clang.dll`.
            "libclang.dll".into()
        } else {
            format!("{}clang{}", env::consts::DLL_PREFIX, env::consts::DLL_SUFFIX)
        };

        // Find the `libclang` shared library.
        let directory = match find(&file, "LIBCLANG_PATH") {
            Some(directory) => directory,
            _ => error(&file, "LIBCLANG_PATH")
        };

        println!("cargo:rustc-link-search={}", directory);

        if cfg!(all(target_os="windows", target_env="msvc")) {

            let lib_file = "libclang.lib";
            
            // Find the `libclang` link library.
            let lib_directory = match find(lib_file, "LIBCLANG_PATH") {
                Some(directory) => directory,
                _ => error(lib_file, "LIBCLANG_PATH")
            };

            println!("cargo:rustc-link-search={}", lib_directory);
            println!("cargo:rustc-link-lib=dylib=libclang");

        } else {
            println!("cargo:rustc-link-lib=dylib=clang");
        };
    }
}
