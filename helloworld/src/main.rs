/**
 *
 * Rust program
 *
 */

fn print_hello() {
    println!("Hello World!")
}

fn main() {
    print_hello();

    let a: i32 = 2;
    let b: &str = "Something hooge";
    println!("{} {}", a, b);

    // Datatypes
    let unsigned_int: u128 = 3930045;
    let signed_int: i128 = -45004;

    let float_num: f32 = 5.69;

    println!(
        "Unsigned: {}, signed: {}, floatNum: {}",
        unsigned_int, signed_int, float_num
    );

    // If else statements
    let n: i8 = 8;

    if n > 2 {
        println!("Good to go")
    } else {
        println!("Not good")
    }
}
