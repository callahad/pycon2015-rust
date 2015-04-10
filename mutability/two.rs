fn add_one(num: &mut i32) {
    *num += 1;
}

fn main() {
    let mut x = 5;

    add_one(&mut x);

    println!("{}", x)
}

