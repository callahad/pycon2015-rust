#![feature(core)]

extern crate core;

use std::thread;
use core::num::Int;

#[no_mangle]
pub extern fn triple(x: i64) -> i64 {

    let _: Vec<_> = (0..10).map(|i| {
        thread::scoped(move || {
            let mut x = 0;
            println!("Thread {} running", i);
            while x < 2.pow(23) {
                x += 1
            }
            println!("Thread {} returning", i);
        })
    }).collect();

    x * 3
}

#[test]
fn it_works() {
    assert!(triple(3) == 9);
}
