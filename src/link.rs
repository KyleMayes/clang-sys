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
    (@IMPL: #[cfg($cfg:meta)] fn $name:ident($($pname:ident: $pty:ty), *) $(-> $ret:ty)*) => (
        #[cfg($cfg)]
        pub fn $name(library: &mut super::SharedLibrary) -> Result<(), String> {
            let symbol = unsafe { library.library.get(stringify!($name).as_bytes()) }.map_err(|_| {
                format!("could not load `{}`", stringify!($name))
            });
            library.functions.$name = *try!(symbol);
            Ok(())
        }

        #[cfg(not($cfg))]
        pub fn $name(_: &mut super::SharedLibrary) -> Result<(), String> {
            Ok(())
        }
    );

    (@IMPL: fn $name:ident($($pname:ident: $pty:ty), *) $(-> $ret:ty)*) => (
        pub fn $name(library: &mut super::SharedLibrary) -> Result<(), String> {
            let symbol = unsafe { library.library.get(stringify!($name).as_bytes()) }.map_err(|_| {
                format!("could not load `{}`", stringify!($name))
            });
            library.functions.$name = *try!(symbol);
            Ok(())
        }
    );

    ($($(#[cfg($cfg:meta)])* pub fn $name:ident($($pname:ident: $pty:ty), *) $(-> $ret:ty)*;)+) => (
        use std::cell::{RefCell};
        use std::sync::{Mutex};

        #[doc(hidden)]
        pub struct Functions {
            $($(#[cfg($cfg)])* $name: extern fn($($pname: $pty), *) $(-> $ret)*), +
        }

        impl Default for Functions {
            fn default() -> Functions {
                unsafe { std::mem::zeroed() }
            }
        }

        #[doc(hidden)]
        pub struct SharedLibrary {
            library: libloading::Library,
            functions: Functions,
        }

        impl SharedLibrary {
            //- Constructors -----------------------------

            fn new(library: libloading::Library) -> SharedLibrary {
                SharedLibrary { library: library, functions: Functions::default() }
            }
        }

        lazy_static!(static ref LOADED: Mutex<bool> = Mutex::new(false););
        thread_local!(static LIBRARY: RefCell<Option<SharedLibrary>> = RefCell::new(None));

        $($(#[cfg($cfg)])* pub unsafe fn $name($($pname: $pty), *) $(-> $ret)* {
            let f = LIBRARY.with(|l| l.borrow().as_ref().map(|l| l.functions.$name));
            (f.expect("a `libclang` shared library was not loaded on this thread"))($($pname), *)
        })+

        /// Loads a `libclang` shared library for use in the current thread.
        ///
        /// # Failures
        ///
        /// * a `libclang` shared library has already been loaded
        /// * a `libclang` shared library could not be found
        /// * a `libclang` shared library symbol could not be loaded
        #[allow(dead_code)]
        pub fn load() -> Result<(), String> {
            #[path="../build.rs"]
            mod build;

            mod load {
                $(link!(@IMPL: $(#[cfg($cfg)])* fn $name($($pname: $pty), *) $(-> $ret)*);)+
            }

            let mut loaded = LOADED.lock().unwrap();
            if *loaded {
                return Err("a `libclang` shared library has already been loaded".into());
            }

            let file = try!(build::find_shared_library().map(|(d, f)| d.join(f)));
            let library = libloading::Library::new(&file).map_err(|_| {
                format!("'{}' could not be opened", file.display())
            });
            let mut library = SharedLibrary::new(try!(library));
            $(try!(load::$name(&mut library));)+
            LIBRARY.with(|l| *l.borrow_mut() = Some(library));
            *loaded = true;
            Ok(())
        }

        /// Unloads the `libclang` shared library in use in the current thread.
        ///
        /// # Failures
        ///
        /// * a `libclang` shared library is not in use in the current thread
        pub fn unload() -> Result<(), String> {
            let mut loaded = LOADED.lock().unwrap();
            LIBRARY.with(|l| {
                let mut library = l.borrow_mut();
                if library.is_some() {
                    *library = None;
                    *loaded = false;
               	    Ok(())
                } else {
                    Err("a `libclang` shared library is not in use in the current thread".into())
                }
            })
        }
    )
}

#[cfg(not(feature="runtime"))]
macro_rules! link {
    ($($(#[cfg($cfg:meta)])* pub fn $name:ident($($pname:ident: $pty:ty), *) $(-> $ret:ty)*;)+) => (
        extern { $($(#[cfg($cfg)])* pub fn $name($($pname: $pty), *) $(-> $ret)*;)+ }
    )
}
