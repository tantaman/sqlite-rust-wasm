use std::ffi::c_void;

#[no_mangle]
pub extern "C" fn testext_commit_hook(_user_data: c_void) -> i32 {
    0
}
