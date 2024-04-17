use std::io::stdin as stdin;

fn main() {
    println!("Give me a sentence: ");

    let mut sentence = String::new();
    stdin().read_line(&mut sentence).expect("Failed to read line");

    let splits = &mut sentence.split_whitespace();

    let mut pig_latin = String::new();

    for word in splits {
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
}

