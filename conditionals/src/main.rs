use std::io;

/**
 * Conditionals with Rust
 */

// If else
fn compare(num1: i16, num2: i16) {
    if num1 > num2 {
        println!("{num1} is greater than {num2}")
    } else if num1 < num2 {
        println!("{num1} is lesser than {num2}",)
    } else {
        println!("{num1} is equal to {num2}")
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
    println!("{num1} {response} {num2}");
}

fn analyze_blood_group(blood_group: &str) -> &str {
    match blood_group {
        "O" => "Can transfer blood to other blood groups, cannot recieve blood from others",
        "A" => "Can transfer blood to group A and group AB, can recieve blood from A and O",
        "B" => "Can transfer blood to group B and group AB, can recieve blood from B and O",
        "AB" => "Can transfer blood to AB, can recieve blood from all blood groups",
        _ => "Cannot recognize this blood group",
    }
}

fn main() {
    println!("Number comparison");
    compare(4, 2);
    compare(2, 78);
    compare(11, 11);
    compare_with_tern(3, 5);

    let mut blood_group = String::new();

    println!("Enter your blood group");
    io::stdin()
        .read_line(&mut blood_group)
        .expect("Failed to read line");

    println!(
        "Group {}, {}",
        blood_group.trim(),
        analyze_blood_group(blood_group.trim())
    );
}
