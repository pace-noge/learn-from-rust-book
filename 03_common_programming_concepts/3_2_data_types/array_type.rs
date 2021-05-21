use std::io;


fn main() {

    let a = [3; 5];

    println!("{:?}", a);

    // accessing array
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{}", first);
    println!("{}", second);

    // invalid element access

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");

    let index: usize = index.trim().parse().expect("Index entered is not number");

    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);
}