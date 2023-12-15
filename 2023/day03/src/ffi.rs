use crate::parser::parse;
use crate::EngineMap;
use std::ffi::{c_char, CStr, CString};

// This is the function we want to import into PHP via FFI.
#[no_mangle]
pub extern "C" fn parse_engine_textmap_to_json(input: *const c_char) -> *const c_char {
    let input = unsafe { CStr::from_ptr(input) }.to_str().unwrap();
    let engine: EngineMap = parse(input)
        .expect("Engine input could not be parsed.")
        .into();
    let json = serde_json::to_string(&engine).unwrap();
    CString::new(json).unwrap().into_raw()
}
