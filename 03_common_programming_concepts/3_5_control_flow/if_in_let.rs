fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    another_if();
}

fn another_if() {
    let condition = true;
    // example of invalid if
    // let number = if condition { 5 } else { "six" };
    // println!("The value of number is: {}", number);
}