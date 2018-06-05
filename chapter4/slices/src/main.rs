fn main() {
    
    let s = String::from("hello world");
    // You can slice Strings
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}", hello);
    println!("{}", first_word("Hello bello"));


    // You can slice lists
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
}

// This is a general approach 
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

