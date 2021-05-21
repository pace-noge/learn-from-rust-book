fn main() {
    let a = [1, 2, 3, 4, 5, 6];
    let slice = &a[1..3];
    println!("{:?}", slice);

    let word = "Hello world";
    println!("{:?}", &word[0..3]);
}