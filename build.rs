use std::env;
use std::path::{Path};
use std::process::{Command};

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
];

const SEARCH_WINDOWS: &'static [&'static str] = &[
    "C:\\Program Files\\LLVM\\bin",
    "C:\\Program Files\\LLVM\\lib",
];

fn error(prefix: &str) -> ! {
    panic!("{:?}, set the LIBCLANG_PATH environment variable to your system's libclang directory", prefix);
}

fn run(command: &str, arguments: &[&str]) -> Option<String> {
    Command::new(command).args(arguments).output().map(|o| {
        String::from_utf8_lossy(&o.stdout).into_owned()
    }).ok()
}

fn find_libclang() -> Option<(String, String)> {
    let search = if let Ok(directory) = env::var("LIBCLANG_PATH") {
        vec![directory]
    } else if let Some(output) = run("llvm-config", &["--libdir"]) {
        vec![output.lines().map(|s| s.to_string()).next().unwrap()]
    } else {
        if cfg!(any(target_os="freebsd", target_os="linux")) {
            SEARCH_LINUX
        } else if cfg!(target_os="macos") {
            SEARCH_OSX
        } else if cfg!(target_os="windows") {
            SEARCH_WINDOWS
        } else {
            error("unsupported operating system");
        }.into_iter().map(|s| s.to_string()).collect()
    };
    let library = format!("{}clang{}", env::consts::DLL_PREFIX, env::consts::DLL_SUFFIX);
    let directory = search.into_iter().find(|d| Path::new(&d).join(&library).exists());
    directory.map(|d| (d, library))
}

const LIBRARIES: &'static [&'static str] = &[
    "LLVMAnalysis",
    "LLVMBitReader",
    "LLVMCore",
    "LLVMLTO",
    "LLVMLinker",
    "LLVMMC",
    "LLVMMCParser",
    "LLVMObjCARCOpts",
    "LLVMObject",
    "LLVMOption",
    "LLVMScalarOpts",
    "LLVMSupport",
    "LLVMTarget",
    "LLVMTransformUtils",
    "LLVMVectorize",
    "LLVMipa",
    "LLVMipo",
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

fn get_libraries() -> Vec<String> {
    run("llvm-config", &["--libs"]).map(|o| {
        o.split_whitespace().filter_map(|p| {
            Path::new(p).file_stem().map(|l| l.to_string_lossy()[2..].into())
        }).collect()
    }).unwrap_or_else(|| LIBRARIES.iter().map(|l| (*l).into()).collect())
}

fn main() {
    let (directory, file) = match find_libclang() {
        Some((directory, file)) => (directory, file),
        _ => error("unable to find libclang"),
    };

    if cfg!(feature="static") {
        print!("cargo:rustc-flags=");
        if let Ok(directory) = env::var("LIBCLANG_STATIC_PATH") {
            print!("-L {} ", directory);
        }
        for library in get_libraries() {
            print!("-l static={} ", library)
        }
        println!("-L {} -l ncursesw -l z -l stdc++", directory);
    } else {
        if cfg!(target_os="linux") {
            println!("cargo:rustc-link-search={}", directory);
            println!("cargo:rustc-link-lib=dylib=:{}", file);
        } else {
            println!("cargo:rustc-link-search={}", directory);
            println!("cargo:rustc-link-lib=dylib=clang");
        }
    }
}
