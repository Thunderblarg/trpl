use std::collections::HashMap;

fn main() {
    // Given a list of integers, use a vector
    println!("Given a list of integers, use a vector:");
    let mut v = vec![4, 6, 2, 9, 1, 7, 3, 5, 8, 2,
                     0, 7, 5, 9, 3, 8, 1, 4, 6, 10,
                     2, 5, 8, 3, 9, 7, 1, 4, 0, 6,
                     10, 9, 2, 5, 8, 1, 3, 7, 4, 0,
                     6, 10, 2, 5, 8, 3, 9, 7, 1, 4];
    println!("vector:{:?}", v);

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
        println!("The number that appears most frequently in the vector is {}", mode.get(&saved_key));
    } else if mode.len() > 1 {
        // If multiple values have the same amount of the most occurences, spit them out as a tie
        let mut occ = String::new();
        let el = mode.len();
        for (idx, (k, v)) in mode.iter().enumerate() {
            occ.push("{}", k.to_string());
            if index > el - 1 {
                occ.push(",")
            }
        }
    }




}
