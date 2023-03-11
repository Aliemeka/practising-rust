use std::collections::HashMap;

fn main() {
    // Vectors
    let mut my_vec: Vec<u64> = vec![1, 2, 45, 899, 900, 1000];
    my_vec.push(18);
    my_vec.remove(2);
    let vec_len = my_vec.len();
    println!("Length of vector: {vec_len}\nVector values are {my_vec:?}");

    // Hashmaps
    let mut map: HashMap<&str, i32> = HashMap::new();

    map.insert("k", 20);
    map.insert("l", 30);
    map.insert("m", 40);

    // Print Hash Map keys
    for key in map.keys() {
        println!("{key}")
    }

    // Print Hash Map values
    for value in map.values() {
        println!("{value}")
    }

    for (key, value) in &map {
        println!("{key}: {value}");
    }

    let mut store_prices: HashMap<&str, u32> =
        HashMap::from([("Chicken", 2500), ("Egg", 120), ("Peak Milk", 140)]);

    store_prices.insert("Buttered Bread", 900);

    println!("Prices for items in my store");

    for (item, price) in &store_prices {
        println!("{item} is NGN {price}")
    }

    println!("Store map: {store_prices:?}");

    // Tuples
    let my_tuple: (u8, f32, i16) = (2, 3.4, -12);
    let (first, second, third) = my_tuple;

    println!("{first} {second} {third}");
}
