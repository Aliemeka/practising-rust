fn calculate(bottom: i32, top: i32) -> i32 {
    (bottom..=top).filter(|n| n % 2 == 0).sum()
}

fn main() {
    let range_sum = calculate(2, 20);
    println!("The range sum of even numbers from 2 to 20 = {range_sum}");
}
