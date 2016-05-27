//! Provides helper functionality.

use std::env;
use std::process::{Command};
use std::path::{Path, PathBuf};

use libc::{c_int};

use super::{CXVersion};

//================================================
// Structs
//================================================

/// A `clang` executable.
#[derive(Clone, Debug)]
pub struct Clang {
    /// The path to this `clang` executable.
    pub path: PathBuf,
    /// The version of this `clang` executable.
    pub version: CXVersion,
    /// The directories searched by this `clang` executable for C headers.
    pub c_search_paths: Vec<PathBuf>,
    /// The directories searched by this `clang` executable for C++ headers.
    pub cpp_search_paths: Vec<PathBuf>,
}

impl Clang {
    //- Constructors -----------------------------

    fn new(path: PathBuf) -> Clang {
        let version = parse_version(&path);
        let c_search_paths = parse_search_paths(&path, "c");
        let cpp_search_paths = parse_search_paths(&path, "c++");
        Clang {
            path: path,
            version: version,
            c_search_paths: c_search_paths,
            cpp_search_paths: cpp_search_paths,
        }
    }

    /// Returns a `clang` executable if one can be found.
    ///
    /// If a path is supplied, that is the first directory searched. Otherwise, the directories in
    /// the system's `PATH` are searched.
    pub fn find(path: Option<&Path>) -> Option<Clang> {
        let clang = format!("clang{}", env::consts::EXE_SUFFIX);
        if let Some(path) = path {
            if contains(path, &clang) {
                return Some(Clang::new(path.join(&clang)));
            }
        }
        for path in env::split_paths(&env::var("PATH").unwrap()) {
            if contains(&path, &clang) {
                return Some(Clang::new(path.join(&clang)));
            }
        }
        None
    }
}

//================================================
// Functions
//================================================

/// Returns whether the supplied directory contains the supplied file.
fn contains(directory: &Path, file: &str) -> bool {
    Path::new(&directory).join(&file).exists()
}

/// Runs a `clang` executable, returning the output.
fn run_clang(path: &Path, arguments: &[&str], stdout: bool) -> String {
    Command::new(path.to_string_lossy().into_owned()).args(arguments).output().map(|o| {
        let output = if stdout {
            &o.stdout
        } else {
            &o.stderr
        };
        String::from_utf8_lossy(output).into_owned()
    }).unwrap()
}

/// Parses the version from the output of a `clang` executable.
fn parse_version(path: &Path) -> CXVersion {
    let output = run_clang(path, &["--version"], true);
    let start = output.find("version ").unwrap() + 8;
    let digits = output[start..].split_whitespace().nth(0).unwrap().split('.');
    let numbers = digits.map(|d| d.parse::<c_int>().unwrap()).collect::<Vec<_>>();
    CXVersion { Major: numbers[0], Minor: numbers[1], Subminor: *numbers.get(2).unwrap_or(&0) }
}

/// Parses the search paths from the output of a `clang` executable.
fn parse_search_paths(path: &Path, language: &str) -> Vec<PathBuf> {
    let output = run_clang(path, &["-E", "-x", language, "-", "-v"], false);
    let start = output.find("#include <...> search starts here:").unwrap() + 34;
    let end = output.find("End of search list.").unwrap();
    output[start..end].split_whitespace().map(|l| Path::new(l.trim()).into()).collect()
}
