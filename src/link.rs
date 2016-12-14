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

//================================================
// Macros
//================================================

// link! _________________________________________

#[cfg(feature="runtime")]
macro_rules! link {
    (@LOAD: #[cfg($cfg:meta)] fn $name:ident($($pname:ident: $pty:ty), *) $(-> $ret:ty)*) => (
        #[cfg($cfg)]
        pub fn $name(library: &mut super::SharedLibrary) {
            let symbol = unsafe { library.library.get(stringify!($name).as_bytes()) }.ok();
            library.functions.$name = symbol.map(|s| *s);
        }

        #[cfg(not($cfg))]
        pub fn $name(_: &mut super::SharedLibrary) {}
    );

    (@LOAD: fn $name:ident($($pname:ident: $pty:ty), *) $(-> $ret:ty)*) => (
        link!(@LOAD: #[cfg(feature="runtime")] fn $name($($pname: $pty), *) $(-> $ret)*);
    );

    ($($(#[cfg($cfg:meta)])* pub fn $name:ident($($pname:ident: $pty:ty), *) $(-> $ret:ty)*;)+) => (
        use std::cell::{RefCell};
        use std::sync::{Arc};

        /// The set of functions loaded dynamically.
        #[derive(Debug)]
        pub struct Functions {
            $($(#[cfg($cfg)])* pub $name: Option<extern fn($($pname: $pty), *) $(-> $ret)*>,)+
        }

        impl Default for Functions {
            fn default() -> Functions {
                unsafe { std::mem::zeroed() }
            }
        }

        /// A dynamically loaded instance of the libclang library.
        #[derive(Debug)]
        pub struct SharedLibrary {
            library: libloading::Library,
            pub functions: Functions,
        }

        impl SharedLibrary {
            //- Constructors -----------------------------

            fn new(library: libloading::Library) -> SharedLibrary {
                SharedLibrary { library: library, functions: Functions::default() }
            }
        }

        thread_local!(static LIBRARY: RefCell<Option<Arc<SharedLibrary>>> = RefCell::new(None));

        /// Whether `libclang` is loaded on this thread.
        pub fn is_loaded() -> bool {
            LIBRARY.with(|l| l.borrow().is_some())
        }

        $(
            $(#[cfg($cfg)])* pub unsafe fn $name($($pname: $pty), *) $(-> $ret)* {
                let f = LIBRARY.with(|l| l.borrow().as_ref().map(|l| {
                    match l.functions.$name {
                        Some(f) => f,
                        None => panic!("Function not loaded: {}!", stringify!($name)),
                    }
                }));
                (f.expect("a `libclang` shared library was not loaded on this thread"))($($pname), *)
            }

            $(#[cfg($cfg)])* pub mod $name {
                use super::LIBRARY;
                pub fn is_loaded() -> bool {
                    LIBRARY.with(|l| l.borrow().as_ref().map_or(false, |l| {
                        l.functions.$name.is_some()
                    }))
                }
            }
        )+

        #[path="../build.rs"]
        mod build;

        mod load {
            $(link!(@LOAD: $(#[cfg($cfg)])* fn $name($($pname: $pty), *) $(-> $ret)*);)+
        }

        /// Loads a `libclang` shared library for use in the current thread.
        ///
        /// # Failures
        ///
        /// * a `libclang` shared library has already been loaded
        /// * a `libclang` shared library could not be found
        ///
        /// Note that this tries to find all the symbols. To check if a symbol
        /// has been found or not, you can use `clang_Foo::is_loaded()`.
        #[allow(dead_code)]
        pub fn load() -> Result<(), String> {
            let lib = Arc::new(try!(load_manually()));
            LIBRARY.with(|l| *l.borrow_mut() = Some(lib));
            Ok(())
        }

        /// Gets the library from tls. This, along with `set_library`, allows
        /// reusing the same library across threads.
        pub fn get_library() -> Option<Arc<SharedLibrary>> {
            LIBRARY.with(|l| {
                l.borrow_mut().clone()
            })
        }

        /// Sets the current library from tls, and returns the previous one.
        pub fn set_library(lib: Option<Arc<SharedLibrary>>) -> Option<Arc<SharedLibrary>> {
            LIBRARY.with(|l| {
                let mut l = l.borrow_mut();
                mem::replace(&mut *l, lib)
            })
        }

        /// Tries to load a libclang library manually, returning the
        /// corresponding `SharedLibrary`.
        ///
        /// Only returns an error when the library couldn't be found or opened,
        /// and the caller is responsible handle the functions manually.
        pub fn load_manually() -> Result<SharedLibrary, String> {
            let file = try!(build::find_shared_library());
            let library = libloading::Library::new(&file).map_err(|_| {
                format!("'{}' could not be opened", file.display())
            });
            let mut library = SharedLibrary::new(try!(library));
            $(load::$name(&mut library);)+

            Ok(library)
        }

        /// Unloads the `libclang` shared library in use in the current thread.
        ///
        /// # Failures
        ///
        /// * a `libclang` shared library is not in use in the current thread
        pub fn unload() -> Result<(), String> {
            let l = set_library(None);

            if l.is_some() {
                Ok(())
            } else {
                Err("a `libclang` shared library is not in use in the current thread".into())
            }
        }
    )
}

#[cfg(not(feature="runtime"))]
macro_rules! link {
    ($($(#[cfg($cfg:meta)])* pub fn $name:ident($($pname:ident: $pty:ty), *) $(-> $ret:ty)*;)+) => (
        extern { $($(#[cfg($cfg)])* pub fn $name($($pname: $pty), *) $(-> $ret)*;)+ }
    )
}
