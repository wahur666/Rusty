fn main() {
    let s1 = String::from("Hello"); // s1 string created from a string literal
    let mut s2 = s1; // s1 value was moved to s2, and s1 is invalidated
    let len = calculate_length(&s2); // s2 passed by reference, to the function does not own it,
                                     // because of this, s2 wont be dropped at he end of the scope.

    println!("The length of '{}' is {}.", s2, len);

    change(&mut s2); // Modifing a borrowed reference only possible if you annotate it

    println!("New String: {}", s2);
    

}

fn calculate_length(s: &String) -> usize { // This reference passing is called borrowing
    s.len()
}

fn change(some_string: &mut String)  { // Mut have to be added to show that you will modify it
    some_string.push_str(", world");
}   