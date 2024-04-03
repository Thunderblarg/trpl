fn main() {
    use std::collections::HashMap;

    // let mut v: Vec<i32> = Vec::new();

    // v.push(4);
    // v.push(6);
    // v.push(9);

    // let v = vec![1, 2, 3];

    // let third: Option<&i32> = v.get(2);

    // println!("{:?}", third);


    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // println!("{}", format!("{s1}-{s2}-{s3}"));

    // let hello = String::from("Здравствуйте");

    // for c in hello.chars() {
    //     println!("{c}");
    // }

    // let hellochars = hello.chars();

    // println!("{:?}", hellochars);

    // println!("{:?}", hello.chars().nth(3));

    // println!("{:?}", hello.chars().nth(22));
    
    let field_name = String::from("Favorite Color");
    let field_values = String::from("Purple");

    let mut map = HashMap::new();

    map.insert(field_name, field_values);

    println!("{:?}", map);

    // println!("{:?}", field_name);
    // println!("{:?}", field_values);

}
