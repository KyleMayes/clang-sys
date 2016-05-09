use std::env;
use std::path::{Path};
use std::process::{Command};

const SEARCH_OSX: &'static [&'static str] = &[
    "/usr/local/opt/llvm/lib",
    "/Library/Developer/CommandLineTools/usr/lib",
    "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib",
];

const SEARCH_WINDOWS: &'static [&'static str] = &[
    "C:\\Program Files\\LLVM\\bin",
    "C:\\Program Files\\LLVM\\lib",
];

fn run(command: &str, arguments: &[&str]) -> Option<String> {
    Command::new(command).args(arguments).output().map(|o| {
        String::from_utf8_lossy(&o.stdout).into_owned()
    }).ok()
}

fn find_libclang() -> Option<(String, String)> {
    let search = if let Ok(directory) = env::var("LIBCLANG_PATH") {
        vec![directory]
    } else {
        run("llvm-config", &["--libdir"]).map(|d| vec![d]).unwrap_or_else(|| {
            if cfg!(target_os="osx") {
                SEARCH_OSX
            } else if cfg!(target_os="windows") {
                SEARCH_WINDOWS
            } else {
                panic!("unsupported operating system!");
            }.into_iter().map(|s| s.to_string()).collect()
        })
    };

    let library = format!("{}clang{}",
                          env::consts::DLL_PREFIX,
                          env::consts::DLL_SUFFIX);

    let directory = search.into_iter().find(|d| Path::new(&d).join(&library).exists());

    if directory.is_none() && cfg!(target_os="linux") {
        if let Some(output) = run("llvm-config", &["--libdir"]) {
            output.lines()
                .next()
                .map(|l| (l.into(), library))
        } else {
            None
        }
    } else {
        directory.map(|d| (d, library))
    }
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
    let (directory, file) = find_libclang().expect("unable to find libclang!");

    if cfg!(feature="static") || env::var("LIBCLANG_STATIC").is_ok() {
        print!("cargo:rustc-flags=");
        if let Ok(directory) = env::var("LIBCLANG_STATIC_PATH") {
            print!("-L {} ", directory);
        }
        for library in get_libraries() {
            print!("-l static={} ", library)
        }
        println!("-L {} -l ncursesw -l z -l stdc++", directory);
    } else {
        println!("cargo:rustc-link-search={}", directory);
        println!("cargo:rustc-link-lib=dylib=:{}", file);
    }
}
