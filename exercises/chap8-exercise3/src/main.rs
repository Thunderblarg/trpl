use std::io::stdin as stdin;
use std::collections::HashMap;

fn main() {
    let mut exit: bool = false;

    while !exit {
        let mut input = String::new();
        let mut departments: HashMap<String, Vec<String>> = HashMap::new();

        println!("Awaiting command: ");
        stdin().read_line(&mut input).expect("Failed to read line");

        let mut input = input.split_whitespace();
        let arg1 = input.next();
        if let Some(String) = arg1 {
            match arg1.unwrap() {
                "exit"      => exit = true,
                "add"       => {
                    let arg2 = input.next();
                    if let Some(String) = arg2 {
                        if arg2.unwrap() != "to" {
                            println!("we're looking at some kind of typo here WHAT ARE YOU EVEN DOING???");
                        } else {
                            // placeholder here
                        }
                    }
                }
                "create"    => {println!("creating something");}
                _ => {
                    println!("got nothing here");
                    continue
                }
            }
        } else {
            println!("it's busted");
            continue;
        }       
    }
}

fn add_department(dept: &mut HashMap<String, Vec<String>>, new_dept: String){}