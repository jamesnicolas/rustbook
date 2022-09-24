#[derive(Debug, Clone, Copy)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    skin: Color,
}

impl User {
    fn brightness(&self) -> i32 {
        self.skin.0 + self.skin.1 + self.skin.2
    }
    fn is_better(&self, other: &Self) -> bool {
        println!("Comparing users {} and {}",
                 self.username,
                 other.username,
        );
        if self.active {
            self.brightness() > other.brightness()
        } else {
            return false
        }
    }
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

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width < other.width && self.height < other.height
    }

    fn square(side: u32) -> Self {
        Rectangle {
            width: side,
            height: side,
        }
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
    user1.username = String::from("a REAL username");
    println!("{:?}",user1.email);
    user1.is_better(&user2);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("(Func) The area of the rectangle is {}", area(&rect1));
    println!("(Meth) The area of the rectangle is {}", rect1.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let _ = Rectangle::square(100);

    dbg!(&rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
}
