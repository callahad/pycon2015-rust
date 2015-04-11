fn main() {
    let y;
    {
        let x = Box::new(5);
        y = &x;
        println!("{}", x);
    }
    println!("{}", y)
}
