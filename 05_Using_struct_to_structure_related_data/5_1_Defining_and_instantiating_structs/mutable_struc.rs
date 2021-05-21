struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main(){
    let user1 = build_user(String::from("someone@example.com"), "someone".to_string());
    println!("{}", user1.email);

    // creating instance from other instance with struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotheruser"),
        active: true,
        sign_in_count: user1.sign_in_count,
    };

    println!("{}, {}", user2.email, user2.sign_in_count);

    // set the remaining field based on another struct
    let user3 = User {
        email: String::from("user3@mailc.om"),
        username: String::from("user3"),
        ..user1
    };

    println!("user3 {}, {}", user3.email, user3.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}