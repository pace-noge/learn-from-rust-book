fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true!");
    } else {
        println!("conditiion was false");
    }

    mismatch_type();
}

fn mismatch_type() {
    let number = 3;

    if number == 3 {
        println!("number was three");
    }
}