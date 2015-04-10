#[allow(dead_code)]
#[no_mangle]
pub extern fn double(x: i32) -> i32 {
    x * 2
}


#[test]
fn it_works() {
    assert!(double(3) == 6)
}
