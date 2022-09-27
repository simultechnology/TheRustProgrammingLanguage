fn main() {
    println!("ch5-a starts");

    let mut user1 = User {
        username: String::from("taka.ishikawa"),
        email: String::from("ishikawa@simultechnology.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("{:?}", user1);

    user1.username = String::from("tissi0708@gmail.com");
    println!("{:?}", user1);

    let user2 = bind_user(String::from("simultechnology@gmail.com"), "hoge-tsugu".to_string());
    println!("{:?}", user2);

    let user3 = User {
        email: String::from("admin@simultechnology.com"),
        username: String::from("kanri-sha"),
        ..user2
    };
    println!("{:?}", user3);
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn bind_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
