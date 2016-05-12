use std::env;
use std::path::{Path};
use std::process::{Command};

// Environment variables:
//
// * LLVM_CONFIG_PATH - provides a path to an `llvm-config` executable
// * LIBCLANG_PATH - provides a path to a libclang dynamic library
// * LIBCLANG_STATIC_PATH - provides a path to LLVM and Clang static libraries

/// Returns whether the supplied directory contains the supplied file.
fn contains(directory: &str, file: &str) -> bool {
    Path::new(&directory).join(&file).exists()
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

const SEARCH_LINUX: &'static [&'static str] = &[
    "/usr/lib",
    "/usr/lib/llvm",
    "/usr/lib/llvm-3.8/lib",
    "/usr/lib/llvm-3.7/lib",
    "/usr/lib/llvm-3.6/lib",
    "/usr/lib/llvm-3.5/lib",
    "/usr/lib/llvm-3.4/lib",
    "/usr/lib64/llvm",
    "/usr/lib/x86_64-linux-gnu",
    "/usr/local/lib",
];

const SEARCH_OSX: &'static [&'static str] = &[
    "/usr/local/opt/llvm/lib",
    "/Library/Developer/CommandLineTools/usr/lib",
    "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib",
    "/usr/local/opt/llvm35/lib/llvm-3.8/lib",
    "/usr/local/opt/llvm35/lib/llvm-3.7/lib",
    "/usr/local/opt/llvm35/lib/llvm-3.6/lib",
    "/usr/local/opt/llvm35/lib/llvm-3.5/lib",
];

const SEARCH_WINDOWS: &'static [&'static str] = &[
    "C:\\Program Files\\LLVM\\bin",
    "C:\\Program Files\\LLVM\\lib",
];

/// Searches for a library, returning the directory it can be found in if the search was successful.
fn find(file: &str, env: &str) -> Option<String> {
    if let Some(directory) = env::var(env).ok() {
        if contains(&directory, file) {
            return Some(directory);
        }
    }
    if let Some(output) = run_llvm_config(&["--libdir"]) {
        let directory = output.lines().map(|s| s.to_string()).next().unwrap();
        if contains(&directory, file) {
            return Some(directory);
        }
    }
    let search = if cfg!(any(target_os="freebsd", target_os="linux")) {
        SEARCH_LINUX
    } else if cfg!(target_os="macos") {
        SEARCH_OSX
    } else if cfg!(target_os="windows") {
        SEARCH_WINDOWS
    } else {
        panic!(
            "unsupported operating system, set the LIBCLANG_PATH environment variable to a path \
            where {} can be found",
            file,
        );
    };
    search.iter().find(|d| contains(d, file)).map(|s| s.to_string())
}

const CLANG_LIBRARIES: &'static [&'static str] = &[
    "clang",
    "clangARCMigrate",
    "clangAST",
    "clangASTMatchers",
    "clangAnalysis",
    "clangBasic",
    "clangDriver",
    "clangEdit",
    "clangFormat",
    "clangFrontend",
    "clangIndex",
    "clangLex",
    "clangParse",
    "clangRewrite",
    "clangRewriteFrontend",
    "clangSema",
    "clangSerialization",
    "clangStaticAnalyzerCheckers",
    "clangStaticAnalyzerCore",
    "clangStaticAnalyzerFrontend",
    "clangTooling",
];

/// Returns the list of LLVM static libraries that need to be linked to to use libclang.
fn get_llvm_libraries() -> Vec<String> {
    run_llvm_config(&["--libs"]).expect(
        "could not execute `llvm-config --libs`, either add `llvm-config` to your system's path or \
         set the LLVM_CONFIG_PATH environment variable to a path to an `llvm-config` executable"
    ).split_whitespace().filter_map(|p| {
        // Depending on the version of `llvm-config` in use, listed libraries may be in one of two
        // forms, a full path to the library or simply prefixed with `-l`.
        if p.starts_with("-l") {
            Some(p[2..].into())
        } else {
            Path::new(p).file_stem().map(|l| l.to_string_lossy()[3..].into())
        }
    }).collect()
}

fn main() {
    if cfg!(feature="static") {
        let directory = match find("libclangLex.a", "LIBCLANG_STATIC_PATH") {
            Some(directory) => directory,
            _ => panic!(
                "could not find LLVM and Clang static libraries, set the LIBCLANG_STATIC_PATH \
                 environment variable to a path where these libraries can be found"
            ),
        };

        print!("cargo:rustc-flags=");
        print!("-L {} ", directory);
        for library in get_llvm_libraries() {
            print!("-l static={} ", library);
        }
        for library in CLANG_LIBRARIES {
            print!("-l static={} ", library);
        }
        if cfg!(any(target_os="freebsd", target_os="linux")) {
            println!("-l ffi -l ncursesw -l stdc++ -l z");
        } else if cfg!(target_os="macos") {
            println!("-l ffi -l ncurses -l stdc++ -l z");
        } else {
            panic!("unsupported operating system for static linking");
        };
    } else {
        // libclang's filename should be `clang.dll` on Windows, but it is `libclang.dll`.
        let file = if cfg!(target_os="windows") {
            "libclang.dll".into()
        } else {
            format!("{}clang{}", env::consts::DLL_PREFIX, env::consts::DLL_SUFFIX)
        };
        let directory = match find(&file, "LIBCLANG_PATH") {
            Some(directory) => directory,
            _ => panic!(
                "could not find `{0}`, set the LIBCLANG_PATH environment variable to a path where \
                {0} can be found",
                file,
            ),
        };

        println!("cargo:rustc-link-search={}", directory);
        println!("cargo:rustc-link-lib=dylib=clang");
    }
}
