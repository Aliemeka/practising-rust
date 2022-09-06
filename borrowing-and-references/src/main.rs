// Borrowing and References

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" so much!");
}

fn main() {
    // Using immutable references
    let s1 = String::from("Some word");
    let len = calculate_length(&s1);

    println!("Length of the string {s1} is {len}");

    // Using mutable references
    let mut s2 = String::from("I love Rust");
    change(&mut s2);
    println!("{s2}")
}
