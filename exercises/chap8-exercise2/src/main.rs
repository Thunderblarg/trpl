// use std::io;
use std::io::stdin as stdin;

fn main() {
    println!("Testing strings\n");

    println!("Give me a string: ");

    let mut test_string = String::new();

    stdin().read_line(&mut test_string).expect("Failed to read line");

    println!("Eaten string: {}", test_string);

    println!("Slice off the first letter: {}", &test_string[0..1]);

    let testchar = &test_string[0..1];

    println!("scope check: {}", testchar);

    let mut splits = &mut test_string.split_whitespace();

    let mut pig_latin = String::new();

    for word in splits {
        // println!("word: {}", word);
        // println!("first letter: {}", &word[0..1]);
        match &word[0..1] {
            "a" | "e" | "i" | "o" | "u" | "y" |
            "A" | "E" | "I" | "O" | "U" | "Y" => {
                pig_latin.push_str(&word);
                pig_latin.push_str("-hay ");
            }
            _ => {
                pig_latin.push_str(&word[1..]);
                pig_latin.push_str("-");
                pig_latin.push_str(&word[0..1]);
                pig_latin.push_str("ay ");
            }
        }
    }
    
    println!("{}", pig_latin);

    // match testchar {
    //     "r" => {println!("It works");},
    //     _ => {println!("ain't workin");}
    // }
}
