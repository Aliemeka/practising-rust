fn main() {
    // Arrays
    let arr: [u32; 3] = [1, 2, 3];
    let o_arr: [u32; 5] = [16; 5];

    println!("{:?}", arr);
    println!("{:?}", o_arr);

    // Tuples
    let tuple_1: (i8, bool, &str) = (2, false, "Think");
    println!("{}, {}, {}", tuple_1.0, tuple_1.1, tuple_1.2);

    // Destructuring tuples
    let (a, b, c) = tuple_1;
    println!("a: {}, b: {}, c: {}", a, b, c);

    println!("Length of arr: {}", arr.len());

    // Slices

    let main_arr: [u32; 5] = [1, 3, 4, 5, 6];
    let slice = &main_arr[1..3];

    println!("Slice: {:?}", slice);

    // Slice from string

    let phrase: &str = "Hello World!";

    let slice_of_phrase = &phrase[..6];

    println!("Slice of {} is {}", phrase, slice_of_phrase);
}
