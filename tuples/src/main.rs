use tuples::{print_diff, reverse_cord, Cords};

fn main() {
    let cords: Cords = Cords(6.3, 15.0);
    print_diff(&cords);

    let reversed_cord = reverse_cord(&cords);
    let new_cord = (cords.0, cords.1);
    println!("Reversed {:?} = {:?}", new_cord, reversed_cord);
}
