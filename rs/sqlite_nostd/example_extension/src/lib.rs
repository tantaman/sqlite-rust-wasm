use sqlite3_nostd;

#[no_mangle]
pub extern "C" fn sqlite3_exampleextension_init(
    db: *mut sqlite3,
    _errMsg: *mut *mut c_char,
    api: *mut sqlite3_api_routines,
) -> u32 {
    sqlite3_nostd::init(api);

    // register a function extension
    // return a string to test allocation

    SQLITE_OK
}
