use std::io::stdin as stdin;
use std::collections::HashMap;

fn main() {
    let mut exit: bool = false;
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    while !exit {
        let mut input = String::new();

        println!("Awaiting command: ");
        stdin().read_line(&mut input).expect("Failed to read line");    

        let mut input = input.split_whitespace();
        let arg1 = input.next();
        if let Some(_string) = arg1 { // verb
            match arg1.unwrap() {
                "exit"      => exit = true,         // exits us out, pretty self explanatory
                "add"       => {                    // Adds a name to an existing department
                    if departments.is_empty() {
                        println!("There are no departments!");
                        continue;
                    }

                    let arg2 = input.next(); // name
                    if let Some(_string) = arg2 {
                        // I hate doing this nesting shit, I really need to look into
                        // a better convention for this
                        let arg3 = input.next(); // anyways, to
                        if arg3.unwrap() != "to" {
                            println!("we're looking at some kind of typo here WHAT ARE YOU EVEN DOING???");
                            continue;
                        } else {
                            // fuuuuUUUCK WHY
                            let arg4 = input.next(); // department
                            if let Some(_string) = arg4 {
                                println!("arg4 = {}", arg4.unwrap());
                                // TIL about get_mut, literally didn't even know this was a thing
                                let some_department = departments.get_mut(arg4.unwrap()); 
                                if let Some(employee_list) = some_department {
                                    println!("Good so far: 40");
                                    println!("arg2.unwrap() = {:?}", arg2.unwrap());
                                    let new_name = String::from(arg2.unwrap());
                                    employee_list.push(new_name);
                                    employee_list.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
                                } else {
                                    println!("Department doesn't exist!");
                                }
                            }
                        }
                    }
                }
                "create"    => { // this creates a new department
                    create_department(&mut departments, input.next());
                }
                "list"      => { // list all 
                    let arg2 = input.next();
                    if let Some(_string) = arg2 {
                        if departments.contains_key(arg2.unwrap()) {
                            for person in &departments.get(arg2.unwrap()) {
                                println!("{:?}", person);
                            }    
                        }
                    } else {
                        for dept in &departments {
                            println!("{:?}", dept);
                        }
                    }
                }
                _ => {
                    println!("got nothing here");
                    continue;
                }
            }
        } else {
            println!("it's busted: 59");
            continue;
        }       
    }
}

fn create_department(dept: &mut HashMap<String, Vec<String>>, new_dept: Option<&str>){
    // let arg2 = input.next();


    if let Some(_string) = new_dept {
        if dept.contains_key(new_dept.unwrap()){
            println!("Department already exists!");
            // continue;
        } else {
            let personnel: Vec<String> = Vec::new();
            dept.insert(String::from(new_dept.unwrap()), personnel);
        }    
    } else {
        println!("Create what?");
    }
}