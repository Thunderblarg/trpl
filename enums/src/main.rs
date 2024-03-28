fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let home = IpAddr::V4(123, 0, 0, 1);

    // let loopback = IpAddr::V6(String::from("::1"));

    // let some_number = Some(5);
    // let come_char = Some('e');

    // let absent_number: Option<i32> = Option::None;

    // value_in_cents(Coin::Quarter(UsState::Alabama));

    let mut count: u8 = 0;
    println!("I have {} state quarters", count);
    println!("value_in_cents is {}",
              value_in_cents(Coin::Quarter(UsState::Alabama), &count));
    count = value_in_cents(Coin::Quarter(UsState::Alabama), &count);
    println!("I now have {} state quarters", count);

}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    SomeOthers
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin, mut count: &u8) -> u8 {
    // match coin {
    //     Coin::Penny => {
    //         println!("Lucky penny!");
    //         1
    //     },
    //     Coin::Nickel => 5,
    //     Coin::Dime => 10,
    //     Coin::Quarter(state) => {
    //         println!("State quarter from {:?}!", state);
    //         25
    //     }
    // }
    
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } // else {
    //     return count + 1
    // }

    *count + 1
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// enum Option<T> {
//     None,
//     Some(T),
// }

// enum IpAddrKind {
//     V4(String),
//     V6(String)
// }

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String)
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32)
// }

// impl Message {
//     fn call(&self) {
//         println!("message called");
//     }
// }

// fn route(ip_kind: IpAddrKind) {}