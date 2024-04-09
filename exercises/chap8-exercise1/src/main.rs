use std::collections::HashMap;

fn main() {
    // Given a list of integers, use a vector
    println!("Given a list of integers, use a vector:");
    let mut v = vec![
        3, 18, 7, 13, 6, 15, 10, 8, 4, 5,
        1, 11, 0, 19, 16, 17, 19, 3, 12, 14,
        9, 18, 5, 1, 14, 11, 7, 3, 2, 6,
        17, 18, 20, 2, 9, 6, 14, 2, 12, 11,
        7, 0, 19, 12, 9, 4, 10, 8, 16, 15
    ];

    println!("The vector we are using is as follows:\n{:?}", v);

    // And return the median when sorted (the value in the middle position)
    println!("And return the median when sorted (the value in the middle position)");
    v.sort();
    println!("vector after sort:{:?}", v);

    println!("value in the middle position of the vector: {}", v[v.len() / 2]);

    let mut tally = HashMap::new();
    for num in v {
        let count = tally.entry(num).or_insert(0);
        *count += 1;
    }

    println!("{:?}", tally);
    
    // Check the count for the element:
    let mut mode: HashMap<i32, i32> = HashMap::new();
    let mut saved_key: i32 = -1;
    // println!("mode.len(): {}", mode.len());
     for (num, inst) in &tally {
        // map: key = number, value = number of instances of that number

        // if the map is empty, throw the k/v pair in there
        if mode.is_empty() {
            mode.insert(num.clone(), inst.clone());
            saved_key = num.clone();
        // otherwise, check if the element has a count bigger than what the map has
        } else if mode.get(&saved_key) < Some(inst) {
            // if it does, reset the map, add the new highest count number
            mode.clear();
            mode.insert(num.clone(), inst.clone());
            saved_key = num.clone();
            // if the count is the same as the existing highest number,
        } else if mode.get(&saved_key) == Some(inst) {
            // throw it into the map
            mode.insert(num.clone(), inst.clone());
        }                   
    }
    // After it's done tallying it all up, print out whatever value has the most occurrences
    if mode.is_empty() {
        println!("Something went wrong! How did we even get here??? :o");
    } else if mode.len() == 1 {
        println!("The number that appears most frequently in the vector is {:#?}", mode.get(&saved_key));
    } else if mode.len() > 1 {
        // If multiple values have the same amount of the most occurences, spit them out as a tie
        let mut occ = String::new();
        let el = mode.len();
        for (idx, (k, _v)) in mode.iter().enumerate() {
            occ.push_str(&k.to_string());
            if idx < el - 2 {
                occ.push_str(", ");
            } else if idx < el - 1 {
                occ.push_str(", and ");
            }
        }
        // println!("{:?}", mode.get(&saved_key)); //:>> Some(5)
        if let Some(_i32) = mode.get(&saved_key) {
            println!("The numbers {} are tied for the most occurrences with {} entries",
                        occ,
                        mode.get(&saved_key).unwrap());
        }
    }
}
