/**
 * Conditionals with Rust
 */

// If else
fn compare(num1: i16, num2: i16) {
    if num1 > num2 {
        println!("{} is greater than {}", num1, num2)
    } else if num1 < num2 {
        println!("{} is lesser than {}", num1, num2)
    } else {
        println!("{} is equal to {}", num1, num2)
    }
}

// Ternary
fn compare_with_tern(num1: i16, num2: i16) {
    let response: &str = if num1 > num2 {
        "is greater than"
    } else if num1 < num2 {
        "is lesser than"
    } else {
        "is equal to"
    };
    println!("{} {} {}", num1, response, num2);
}

fn main() {
    println!("Number comparison");
    compare(4, 2);
    compare(2, 78);
    compare(11, 11);
    compare_with_tern(3, 5);
}
