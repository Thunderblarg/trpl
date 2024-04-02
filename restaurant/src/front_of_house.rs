pub mod hosting;

mod serving;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    // TODO: Hit the trpl discord and see why the book says to use
    // front_of_house::hosting::add_to_waitlist instead of how I have it
    // below, as it's not compiling for me as-is in the book
    hosting::add_to_waitlist();
}