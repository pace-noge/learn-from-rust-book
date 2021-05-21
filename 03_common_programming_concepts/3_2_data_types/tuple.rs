fn main() {
    let tup: (u32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);

    // access tuple by using period

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("{}", five_hundred);
    println!("{}", six_point_four);
    println!("{}", one);
}