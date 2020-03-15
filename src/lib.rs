mod fence;
mod location;
mod singlebulletproof;

use singlebulletproof::SingleBulletProof;
use location::Location;
use fence::Fence;
use chrono::prelude::*;
use chrono::Duration;
use std::io::Read;
use serde_json;
use serde::Deserialize;

use std::os::raw::{c_char, c_uchar};
use std::ffi::{CString, CStr};

#[derive(Deserialize)]
struct TestJson{
    value: u8,
}

#[no_mangle]
// pub extern fn rust_hello(to: *const c_char) -> *mut Vec<c_uchar> {
// pub extern fn rust_hello(to: *const c_char) -> *mut c_uchar {
pub extern fn rust_hello(to: *const c_char) -> Vec<c_uchar> {
    let c_str = unsafe { CStr::from_ptr(to) };
    let value = match c_str.to_str() {
        Err(_) => 0u8,
        Ok(string) => {
		let data: TestJson = serde_json::from_str(string).unwrap();
		data.value
	},
    };
    // CString::new("Hello ".to_owned() + recipient).unwrap().into_raw()
    // let result = Box::new([value;640].to_vec());
    // Box::into_raw(result)
    // [value;640].to_vec().as_mut_ptr()
    [value;640].to_vec()
}

// #[no_mangle]
// pub extern fn rust_hello_free(s: *mut c_uchar) {
//     unsafe {
//         if s.is_null() { return }
//         // CString::from_raw(s)
// 	Vec::from_raw_parts(s, 640, 640);
//     };
// }


#[cfg(test)]
mod tests;
