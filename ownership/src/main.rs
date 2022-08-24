fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{s2}, world"); // Imma fine with this

    println!("{s1}, world"); // Ehen! Now we're talking
}
