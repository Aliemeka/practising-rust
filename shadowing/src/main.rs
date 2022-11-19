fn main() {
    let age: &str = "25";
    let mut age: u32 = age.trim().parse().expect("Failed to parse");
    println!("{}", age);
}
