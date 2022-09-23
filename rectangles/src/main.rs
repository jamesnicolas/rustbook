#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    skin: Color,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
        skin: Color(0,0,0),
    }
}

fn main() {
    let mut user1 = build_user(String::from("hi@bye.com"), String::from("bye@hi.com"));
    user1.sign_in_count += 1;
    user1.email = String::from("you@floorrip.xyz");

    let user2 = User {
        email: String::from("asdf@floorrip.xyz"),
        ..user1
    };
    println!("{:?}",user1.email);
}
