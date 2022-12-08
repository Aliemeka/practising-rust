fn main() {
    let config_max = Some(8u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
