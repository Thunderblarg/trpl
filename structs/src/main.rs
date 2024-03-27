fn main() {
    // let mut user1 = User {
    //     active: true,
    //     username: String::from("heyitsausername"),
    //     email: String::from("username@rustacean.com"),
    //     sign_in_count: 1
    // };

    // user1.email = String::from("anotheremail@example.com");

    // let user2 = User {
    //     active: user1.active,
    //     ..user1
    // };

    // let subject = AlwaysEqual;

    // let width1 = 30;
    // let height1 = 50;
    
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    // println!("rect1 is {:#?}", rect1);
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// struct AlwaysEqual;

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1
//     }
// }