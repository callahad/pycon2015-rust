fn add_one(num: &mut i32) {
    *num += 1;
}

fn main() {
    let mut x = 5;

    let y = &mut x;
    add_one(y);

    println!("{}", x)
}
