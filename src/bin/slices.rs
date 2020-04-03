fn main() {
    let s = String::from("Hello there");

    let first_word = get_first_word(&s);

    println!("{}", first_word);

    // more string slices
    let hello = &s[..5]; // [..5] is equal to [0..5] because .. will start with the first element
    let there = &s[5..];

    println!("{}{}", hello, there)
}

// returns the first word of a string as a string slice
fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{ // If item is equal to a space (means end of word is reached)
            return &s[..i] // returns first word string slice
        }
    }
    &s[..]
}
