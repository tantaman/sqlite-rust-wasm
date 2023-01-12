#![no_std]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_uchar, c_void, CStr};
use core::str::Utf8Error;
use sqlite3_capi::bindings;

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

pub fn value_bytes(value: *mut bindings::sqlite3_value) -> i32 {
    unsafe { sqlite3_capi::value_bytes(value) }
}

pub fn value_blob<'a>(value: *mut bindings::sqlite3_value) -> &'a [u8] {
    let n = value_bytes(value);
    let b = unsafe { sqlite3_capi::value_blob(value) };
    return unsafe { core::slice::from_raw_parts(b.cast::<u8>(), n as usize) };
}

pub fn value_text<'a>(value: *mut bindings::sqlite3_value) -> Result<&'a str, Utf8Error> {
    let len = value_bytes(value);
    let bytes = unsafe { sqlite3_capi::value_text(value) };
    let slice = unsafe { core::slice::from_raw_parts(bytes as *const u8, len as usize) };
    core::str::from_utf8(slice)
}

pub fn value_int(value: *mut bindings::sqlite3_value) -> i32 {
    unsafe { sqlite3_capi::value_int(value) }
}

pub fn value_int64(value: *mut bindings::sqlite3_value) -> i64 {
    unsafe { sqlite3_capi::value_int64(value) }
}

pub fn value_double(value: *mut bindings::sqlite3_value) -> f64 {
    unsafe { sqlite3_capi::value_double(value) }
}

pub fn result_int(context: *mut bindings::sqlite3_context, i: i32) {
    unsafe { sqlite3_capi::result_int(context, i) };
}

pub fn result_int64(context: *mut bindings::sqlite3_context, i: i64) {
    unsafe { sqlite3_capi::result_int64(context, i) };
}

pub fn result_double(context: *mut bindings::sqlite3_context, i: f64) {
    unsafe { sqlite3_capi::result_double(context, i) };
}

pub fn result_blob(context: *mut bindings::sqlite3_context, blob: &[u8]) {
    // TODO try_into(), err on too big (check against limit? idk)
    let len = blob.len() as c_int;
    unsafe { sqlite3_capi::result_blob(context, blob.as_ptr().cast::<c_void>(), len) };
}

pub fn result_null(context: *mut bindings::sqlite3_context) {
    unsafe { sqlite3_capi::result_null(context) };
}

// pub fn result_error(context: *mut bindings::sqlite3_context, text: &str) -> Result<(), Error> {
//     let s = CStr::new(text.as_bytes())?;
//     let n = text.len() as i32;

//     unsafe { sqlite3_capi::result_error(context, s.into_raw(), n) };
//     Ok(())
// }

pub fn result_error_code(context: *mut bindings::sqlite3_context, code: i32) {
    unsafe { sqlite3_capi::result_error_code(context, code) };
}

pub fn result_bool(context: *mut bindings::sqlite3_context, value: bool) {
    if value {
        result_int(context, 1)
    } else {
        result_int(context, 0)
    }
}
