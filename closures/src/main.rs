fn main() {
    let add = |x, y| x + y;

    println!("{}", add(1, 2));

    let name = "Emeka".to_string();
    let print_name = || {
        println!("{}", name);
    };
    print_name();
}
