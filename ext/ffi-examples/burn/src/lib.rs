use std::thread;

#[no_mangle]
pub extern fn triple(x: i32) -> i32 {

    let _: Vec<_> = (0..10).map(|i| {
        thread::scoped(move || {
            println!("Thread {} running", i);

            let mut x = 0;
            while x < 1_000_000_000 {
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
