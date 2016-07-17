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

//! Provides helper functionality.

use std::env;
use std::process::{Command};
use std::path::{Path, PathBuf};

use glob;

use libc::{c_int};

use super::{CXVersion};

//================================================
// Structs
//================================================

// try_opt! ______________________________________

macro_rules! try_opt {
    ($option:expr) => ({
        match $option {
            Some(some) => some,
            None => return None,
        }
    });
}

//================================================
// Structs
//================================================

/// A `clang` executable.
#[derive(Clone, Debug)]
pub struct Clang {
    /// The path to this `clang` executable.
    pub path: PathBuf,
    /// The version of this `clang` executable if it could be parsed.
    pub version: Option<CXVersion>,
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
        let default = format!("clang{}", env::consts::EXE_SUFFIX);
        let versioned = format!("clang-[0-9]*{}", env::consts::EXE_SUFFIX);
        let patterns = &[&default[..], &versioned[..]];
        if let Some(path) = path.and_then(|p| find(p, patterns)) {
            return Some(Clang::new(path));
        }
        for path in env::split_paths(&env::var("PATH").unwrap()) {
            if let Some(path) = find(&path, patterns) {
                return Some(Clang::new(path));
            }
        }
        None
    }
}

//================================================
// Functions
//================================================

/// Returns the first match to the supplied glob patterns in the supplied directory if there are any
/// matches.
fn find(directory: &Path, patterns: &[&str]) -> Option<PathBuf> {
    for pattern in patterns {
        let pattern = directory.join(pattern).to_string_lossy().into_owned();
        if let Some(path) = try_opt!(glob::glob(&pattern).ok()).filter_map(|p| p.ok()).next() {
            return Some(path);
        }
    }
    None
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

/// Parses a version number if possible, ignoring trailing non-digit characters.
fn parse_version_number(number: &str) -> Option<c_int> {
    number.chars().take_while(|c| c.is_digit(10)).collect::<String>().parse().ok()
}

/// Parses the version from the output of a `clang` executable if possible.
fn parse_version(path: &Path) -> Option<CXVersion> {
    let output = run_clang(path, &["--version"], true);
    let start = try_opt!(output.find("version ")) + 8;
    let mut numbers = try_opt!(output[start..].split_whitespace().nth(0)).split('.');
    let major = try_opt!(numbers.next().and_then(parse_version_number));
    let minor = try_opt!(numbers.next().and_then(parse_version_number));
    let subminor = numbers.next().and_then(parse_version_number).unwrap_or(0);
    Some(CXVersion { Major: major, Minor: minor, Subminor: subminor })
}

/// Parses the search paths from the output of a `clang` executable.
fn parse_search_paths(path: &Path, language: &str) -> Vec<PathBuf> {
    let output = run_clang(path, &["-E", "-x", language, "-", "-v"], false);
    let include_start = "#include <...> search starts here:";
    let start = output.find(include_start).expect(include_start) + include_start.len();
    let end = output.find("End of search list.").expect("End of search list");
    let paths = output[start..end].replace("(framework directory)", "");
    paths.split_whitespace().map(|l| Path::new(l.trim()).into()).collect()
}
