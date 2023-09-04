mod front_of_house;

mod back_of_house {
    fn fix_correct_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // abs path
    crate::front_of_house::hosting::add_to_waitlist();

    // rel path
    front_of_house::hosting::add_to_waitlist();
}
