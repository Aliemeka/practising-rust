pub struct Cords(pub f32, pub f32);

pub fn print_diff(cords: &Cords) {
    let Cords(num1, num2) = cords;
    let diff = (num1 - num2).abs();
    println!("The difference between {num1} and {num2} is {diff}")
}

pub fn reverse_cord(cords: &Cords) -> (f32, f32) {
    return (cords.1, cords.0);
}
