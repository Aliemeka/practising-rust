mod house {
    pub const HOUSE_NUMBER: u32 = 20;
    pub mod bedroom {
        pub fn is_light_on() -> bool {
            false
        }

        pub fn is_neighbours_light_on() -> bool {
            use super::bedroom2;
            bedroom2::is_light_on()
        }
    }
    pub mod bedroom2 {
        pub fn is_light_on() -> bool {
            true
        }
    }
}

mod shop;
use house::{bedroom, bedroom2};
use shop::{store, store2};

mod city;
use city::{house1, house2, NUMBER_OF_HOUSES};

fn main() {
    println!("Hello, world!");
    println!("{}", bedroom::is_light_on());
    println!("{}", bedroom2::is_light_on());
    println!(
        "Is neighbours light on {}",
        bedroom::is_neighbours_light_on()
    );
    println!("House number: {}", house::HOUSE_NUMBER);

    println!("Store number: {}", shop::STORE_NUMBER);
    println!(
        "Store open: {}, neighbours store open: {}",
        store::is_store_open(),
        store::is_neighbours_open()
    );

    println!("Store 2 open {}", store2::is_store_open());

    println!("Number of houses: {}", NUMBER_OF_HOUSES);
    println!("Is House1 sold: {}", house1::is_house_sold());
    println!(
        "Is House1 available: {}. What about House 2 {}",
        house1::is_house_available(),
        house1::is_next_house_for_sale()
    );
    println!("House2 sold: {}", house2::is_house_sold());
}
