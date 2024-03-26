fn main() {
    let foo = String::from("foo");

    // let bar = 420;

    ownership_check(foo.clone());

    println!("{foo}");

}

fn ownership_check(some_string: String){
    println!("{}", some_string);
}
