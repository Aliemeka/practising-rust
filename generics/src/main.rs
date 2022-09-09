// Generic function
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    // Use generic functions to get the largest of a dataset
    let number_list = vec![34, 28, 758, 42, 78, 820, -847];
    let largest_number = largest(number_list);
    println!("The largest number is {largest_number}");

    let character_list = vec!['a', 'A', 'b', '2', 'Z', 'z', '😅'];
    let largest_char = largest(character_list);
    println!("The largest character is {largest_char}");
}
