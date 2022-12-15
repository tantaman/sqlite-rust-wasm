#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn sqlite3_test_extension_init(
    db: *const sqlite3,
    errMsg: *mut *mut c_char,
    api: *const sqlite3_api_routines,
) -> u32 {
    SQLITE_OK
}
