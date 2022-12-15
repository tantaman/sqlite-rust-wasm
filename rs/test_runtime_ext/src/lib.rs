#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use core::ffi::c_char;

use std::ffi::CString;
use std::ptr;

// macro emulation: https://github.com/asg017/sqlite-loadable-rs/blob/main/src/ext.rs
static mut SQLITE3_API: *mut sqlite3_api_routines = std::ptr::null_mut();

static EXPECT_MESSAGE: &str =
    "sqlite-loadable error: expected method on SQLITE3_API. Please file an issue";

#[no_mangle]
pub extern "C" fn testext_fn(
    ctx: *mut sqlite3_context,
    _argc: i32,
    _argv: *mut *mut sqlite3_value,
) {
    unsafe {
        // sqlite3_result_int(ctx, 100);
        ((*SQLITE3_API).result_int.expect(EXPECT_MESSAGE))(ctx, 100);
    }
}

#[no_mangle]
pub extern "C" fn sqlite3_test_runtime_ext_init(
    db: *mut sqlite3,
    _errMsg: *mut *mut c_char,
    api: *mut sqlite3_api_routines,
) -> u32 {
    unsafe {
        let cname = CString::new("testext_fn").expect("Could not allocate function name");
        SQLITE3_API = api;
        ((*SQLITE3_API).create_function.expect(EXPECT_MESSAGE))(
            db,
            cname.as_ptr(),
            0,
            i32::try_from(SQLITE_UTF8 | SQLITE_INNOCUOUS)
                .expect("Failed converting function flags"),
            ptr::null_mut(),
            Some(testext_fn),
            None,
            None,
        );
    }

    SQLITE_OK
}
