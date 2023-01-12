#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_uint};
use core::ptr;
use core::result::Result;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// macro emulation: https://github.com/asg017/sqlite-loadable-rs/blob/main/src/ext.rs
static mut SQLITE3_API: *mut sqlite3_api_routines = ptr::null_mut();

pub unsafe fn faux_sqlite_extension_init2(api: *mut sqlite3_api_routines) {
    SQLITE3_API = api;
}

pub fn SQLITE_EXTENSION_INIT2(api: *mut sqlite3_api_routines) {
    unsafe {
        faux_sqlite_extension_init2(api);
    }
}

/// we don't need this... right? It's just overcomplicating what only need to be a call to `SQLITE_EXTENSION_INIT2`
pub fn start_extension<F>(
    db: *mut sqlite3,
    _pz_err_msg: *mut *mut c_char,
    p_api: *mut sqlite3_api_routines,
    callback: F,
) -> c_uint
where
    F: Fn(*mut sqlite3) -> Result<(), Error>,
{
    unsafe {
        faux_sqlite_extension_init2(p_api);
    }
    match callback(db) {
        Ok(()) => SQLITE_OK,
        Err(err) => err.code(),
    }
}

pub struct Error(ErrorKind);

pub enum ErrorKind {
    ERROR(u32),
}

impl Error {
    pub fn code(&self) -> u32 {
        match self.0 {
            ErrorKind::ERROR(code) => code,
        }
    }
}
