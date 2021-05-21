#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loop_back = IpAddr::V6(String::from("::1"));

    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loop_back);
}