fn add_one(num: &mut i32) {
    *num += 1;
}

fn main() {
    let mut x = 5;

    let y = &mut x;
    add_one(y);

    println!("{}", x)
    // You can't read data that has a mutable pointer to it in scope.
    // This won't compile because y is still in scope.
    // Wrap the two middle lines in a {} block to make this work.
}
