// References
#[derive(Debug)]
struct SomeStruct {
    num: i32,
}

fn print_some_struct(sample_struct: &SomeStruct) {
    println!("{:?}", sample_struct)
}

fn mutate_struct(sample_struct: &mut SomeStruct) {
    sample_struct.num = 8;
}

// Lifetimes
fn biggest_num_struct<'a>(a: &'a SomeStruct, b: &'a SomeStruct) -> &'a SomeStruct {
    if a.num > b.num {
        a
    } else {
        b
    }
}

fn main() {
    let mut sample_struct: SomeStruct = SomeStruct { num: 10 };
    mutate_struct(&mut sample_struct);
    print_some_struct(&sample_struct);
    println!("{:?}", sample_struct);
    let bigger: &SomeStruct;
    let new_struct: SomeStruct = SomeStruct { num: 5 };
    bigger = biggest_num_struct(&sample_struct, &new_struct);
    print_some_struct(bigger);
}
