// SPDX-License-Identifier: Apache-2.0

//================================================
// Macros
//================================================

#[cfg(feature = "runtime")]
macro_rules! link {
    (
        @LOAD:
        $(#[doc=$doc:expr])*
        #[cfg($cfg:meta)]
        fn $name:ident($($pname:ident: $pty:ty), *) $(-> $ret:ty)*
    ) => (
        $(#[doc=$doc])*
        #[cfg($cfg)]
        pub fn $name(library: &mut super::SharedLibrary) {
            let symbol = unsafe { library.library.get(stringify!($name).as_bytes()) }.ok();
            library.functions.$name = match symbol {
                Some(s) => *s,
                None => None,
            };
        }

        #[cfg(not($cfg))]
        pub fn $name(_: &mut super::SharedLibrary) {}
    );

    (
        @LOAD:
        fn $name:ident($($pname:ident: $pty:ty), *) $(-> $ret:ty)*
    ) => (
        link!(@LOAD: #[cfg(feature = "runtime")] fn $name($($pname: $pty), *) $(-> $ret)*);
    );

    (
        $(
            $(#[doc=$doc:expr] #[cfg($cfg:meta)])*
            pub fn $name:ident($($pname:ident: $pty:ty), *) $(-> $ret:ty)*;
        )+
    ) => (
        use std::cell::{RefCell};
        use std::rc::{Rc};
        use std::path::{Path, PathBuf};

        /// The (minimum) version of a `libclang` shared library.
        #[allow(missing_docs)]
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Version {
            V3_5 = 35,
            V3_6 = 36,
            V3_7 = 37,
            V3_8 = 38,
            V3_9 = 39,
            V4_0 = 40,
            V5_0 = 50,
            V6_0 = 60,
            V7_0 = 70,
            V8_0 = 80,
            V9_0 = 90,
        }

        /// The set of functions loaded dynamically.
        #[derive(Debug, Default)]
        pub struct Functions {
            $(
                $(#[doc=$doc] #[cfg($cfg)])*
                pub $name: Option<unsafe extern fn($($pname: $pty), *) $(-> $ret)*>,
            )+
        }

        /// A dynamically loaded instance of the `libclang` library.
        #[derive(Debug)]
        pub struct SharedLibrary {
            library: libloading::Library,
            path: PathBuf,
            pub functions: Functions,
        }

        impl SharedLibrary {
            fn new(library: libloading::Library, path: PathBuf) -> Self {
                Self { library, path, functions: Functions::default() }
            }

            /// Returns the path to this `libclang` shared library.
            pub fn path(&self) -> &Path {
                &self.path
            }

            /// Returns the (minimum) version of this `libclang` shared library.
            ///
            /// If this returns `None`, it indicates that the version is too old
            /// to be supported by this crate (i.e., `3.4` or earlier). If the
            /// version of this shared library is more recent than that fully
            /// supported by this crate, the most recent fully supported version
            /// will be returned.
            pub fn version(&self) -> Option<Version> {
                macro_rules! check {
                    ($fn:expr, $version:ident) => {
                        if self.library.get::<unsafe extern fn()>($fn).is_ok() {
                            return Some(Version::$version);
                        }
                    };
                }

                unsafe {
                    check!(b"clang_Cursor_isAnonymousRecordDecl", V9_0);
                    check!(b"clang_Cursor_getObjCPropertyGetterName", V8_0);
                    check!(b"clang_File_tryGetRealPathName", V7_0);
                    check!(b"clang_CXIndex_setInvocationEmissionPathOption", V6_0);
                    check!(b"clang_Cursor_isExternalSymbol", V5_0);
                    check!(b"clang_EvalResult_getAsLongLong", V4_0);
                    check!(b"clang_CXXConstructor_isConvertingConstructor", V3_9);
                    check!(b"clang_CXXField_isMutable", V3_8);
                    check!(b"clang_Cursor_getOffsetOfField", V3_7);
                    check!(b"clang_Cursor_getStorageClass", V3_6);
                    check!(b"clang_Type_getNumTemplateArguments", V3_5);
                }

                None
            }
        }

        thread_local!(static LIBRARY: RefCell<Option<Rc<SharedLibrary>>> = RefCell::new(None));

        /// Returns whether a `libclang` shared library is loaded on this thread.
        pub fn is_loaded() -> bool {
            LIBRARY.with(|l| l.borrow().is_some())
        }

        fn with_library<T, F>(f: F) -> Option<T> where F: FnOnce(&SharedLibrary) -> T {
            LIBRARY.with(|l| {
                match l.borrow().as_ref() {
                    Some(library) => Some(f(&library)),
                    _ => None,
                }
            })
        }

        $(
            #[cfg_attr(feature="cargo-clippy", allow(clippy::missing_safety_doc))]
            #[cfg_attr(feature="cargo-clippy", allow(clippy::too_many_arguments))]
            $(#[doc=$doc] #[cfg($cfg)])*
            pub unsafe fn $name($($pname: $pty), *) $(-> $ret)* {
                let f = with_library(|l| {
                    l.functions.$name.expect(concat!(
                        "`libclang` function not loaded: `",
                        stringify!($name),
                        "`. This crate requires that `libclang` 3.9 or later be installed on your ",
                        "system. For more information on how to accomplish this, see here: ",
                        "https://rust-lang.github.io/rust-bindgen/requirements.html#installing-clang-39"))
                }).expect("a `libclang` shared library is not loaded on this thread");
                f($($pname), *)
            }

            $(#[doc=$doc] #[cfg($cfg)])*
            pub mod $name {
                pub fn is_loaded() -> bool {
                    super::with_library(|l| l.functions.$name.is_some()).unwrap_or(false)
                }
            }
        )+

        mod load {
            $(link!(@LOAD: $(#[cfg($cfg)])* fn $name($($pname: $pty), *) $(-> $ret)*);)+
        }

        /// Loads a `libclang` shared library and returns the library instance.
        ///
        /// This function does not attempt to load any functions from the shared library. The caller
        /// is responsible for loading the functions they require.
        ///
        /// # Failures
        ///
        /// * a `libclang` shared library could not be found
        /// * the `libclang` shared library could not be opened
        ///
        /// # Safety
        ///
        /// `libclang` must be loaded at most once per thread.
        pub unsafe fn load_manually() -> Result<SharedLibrary, String> {
            mod build {
                pub mod common { include!(concat!(env!("OUT_DIR"), "/common.rs")); }
                pub mod dynamic { include!(concat!(env!("OUT_DIR"), "/dynamic.rs")); }
            }

            let (directory, filename) = build::dynamic::find(true)?;
            let path = directory.join(filename);

            unsafe {
                let library = libloading::Library::new(&path).map_err(|e| {
                    format!(
                        "the `libclang` shared library at {} could not be opened: {}",
                        path.display(),
                        e,
                    )
                });

                let mut library = SharedLibrary::new(library?, path);
                $(load::$name(&mut library);)+
                Ok(library)
            }
        }

        /// Loads a `libclang` shared library for use in the current thread.
        ///
        /// This functions attempts to load all the functions in the shared library. Whether a
        /// function has been loaded can be tested by calling the `is_loaded` function on the
        /// module with the same name as the function (e.g., `clang_createIndex::is_loaded()` for
        /// the `clang_createIndex` function).
        ///
        /// # Failures
        ///
        /// * a `libclang` shared library could not be found
        /// * the `libclang` shared library could not be opened
        #[allow(dead_code)]
        pub fn load() -> Result<Rc<SharedLibrary>, String> {
            LIBRARY.with(|l| {
                if let Some(library) = l.borrow().as_ref() {
                    // Already loaded.
                    return Ok(library.clone())
                }

                let library = Rc::new(unsafe {
                  load_manually()?
                });
                *l.borrow_mut() = Some(library.clone());
                Ok(library)
            })
        }

        /// Unloads the `libclang` shared library in use in the current thread.
        ///
        /// # Failures
        ///
        /// * the `libclang` library is still referenced by the current thread
        /// * the `libclang` shared library is not in use in the current thread
        pub fn unload() -> Result<(), String> {
            LIBRARY.with(|l| {
                let mut l = l.borrow_mut();
                if let Some(library) = l.as_ref() {
                    let strong_count = Rc::strong_count(library);
                    if strong_count == 1 {
                        *l = None;
                        Ok(())
                    } else {
                        Err("the `libclang` shared library is still referenced by the current thread".into())
                    }
                } else {
                    Err("the `libclang` shared library is not in use in the current thread".into())
                }
            })
        }
    )
}

#[cfg(not(feature = "runtime"))]
macro_rules! link {
    (
        $(
            $(#[doc=$doc:expr] #[cfg($cfg:meta)])*
            pub fn $name:ident($($pname:ident: $pty:ty), *) $(-> $ret:ty)*;
        )+
    ) => (
        extern {
            $(
                $(#[doc=$doc] #[cfg($cfg)])*
                pub fn $name($($pname: $pty), *) $(-> $ret)*;
            )+
        }

        $(
            $(#[doc=$doc] #[cfg($cfg)])*
            pub mod $name {
                pub fn is_loaded() -> bool { true }
            }
        )+
    )
}
