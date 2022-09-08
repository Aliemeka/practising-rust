use std::str;

fn main() {
    // String slice
    let s = "Rust Rules Yah Goat!";
    let fw = first_word(s);
    let sw = second_word(s);

    println!("The first word of '{s}' is '{fw}'");
    println!("The second word of '{s}' is '{sw}'");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            let start_index = i + 1;
            for (j, &ntem) in bytes[start_index..s.len()].iter().enumerate() {
                if ntem == b' ' {
                    let end_index = start_index + j;
                    return &s[start_index..end_index];
                }
            }
            return &s[start_index..];
        }
    }

    &s[..]
}
