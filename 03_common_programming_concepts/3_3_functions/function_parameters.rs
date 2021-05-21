fn main() {
    another_function(5);
    other_function(5, 6);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn other_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
