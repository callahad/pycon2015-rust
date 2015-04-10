use std::thread;

#[no_mangle]
pub extern fn triple(x: i32) -> i32 {

    let _: Vec<_> = (0..10).map(|i| {
        thread::scoped(move || {
            let mut x = 0;
            println!("Thread {} running", i);
            while x < 10_000_000 { // Try 1_000_000_000 and 10_000_000

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
