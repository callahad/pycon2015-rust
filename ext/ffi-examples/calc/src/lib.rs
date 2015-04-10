// {{{ We'll look at this later...
#![allow(dead_code)]

extern crate libc;

use libc::c_char;
use std::ffi::CStr;
use std::str;

// {{{ fn calculate(c_buf: *const c_car) -> i64 {...}
#[no_mangle]
pub extern fn calculate(c_buf: *const c_char) -> i64 {
    let buf = unsafe { CStr::from_ptr(c_buf).to_bytes() };
    let slice = str::from_utf8(buf).unwrap();
    calc(slice)
} // }}}
// }}}

fn calc(script: &str) -> i64 {
    let mut accumulator = 0;

    for c in script.chars() {
        match c {
            '+' => accumulator += 1,
            '-' => accumulator -= 1,
            '*' => accumulator *= 2,
            '/' => accumulator /= 2,
            _ => { /* ignore other characters */ }
        }
    }

    accumulator
}

#[test]
fn it_works() {
    assert!(calc("+ + * - /") == 1)
}

// vim: foldmethod=marker:foldenable
