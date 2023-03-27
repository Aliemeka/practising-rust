fn sum_evens(bottom: i32, top: i32) -> i32 {
    (bottom..=top).filter(|n| n % 2 == 0).sum()
}

fn sum_odds(bottom: i32, top: i32) -> i32 {
    (bottom..=top).filter(|n| n % 2 != 0).sum()
}
fn main() {
    let bottom = 2;
    let top = 20;
    let even_sum = sum_evens(bottom, top);
    let odd_sum = sum_odds(bottom, top);
    println!("The range sum of even numbers from 2 to 20 = {even_sum}");
    println!("The range sum of even numbers from 2 to 20 = {odd_sum}");
}
