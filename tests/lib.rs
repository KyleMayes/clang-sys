extern crate clang_sys;
extern crate libc;

use std::ptr;

use clang_sys::*;

use libc::{c_char};

#[test]
fn test() {
    unsafe {
        let index = clang_createIndex(0, 0);
        assert!(!index.0.is_null());

        let tu = clang_parseTranslationUnit(
            index,
            "tests/header.h\0".as_ptr() as *const c_char,
            ptr::null_mut(),
            0,
            ptr::null_mut(),
            0,
            CXTranslationUnit_Flags::empty(),
        );
        assert!(!tu.0.is_null());
    }
}

#[test]
fn test_support() {
    let clang = support::Clang::find(None).unwrap();
    println!("{:?}", clang);
}
