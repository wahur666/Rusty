fn main() {
    // If a struct is mutable, every field is mutable inside
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    user1.email = String::from("anotheremail@example.com"); // dot operator for accessing fields
    user1.active = false;

    let user2 = User {
        email: String::from("Another@example.com"),
        username: String::from("anotherusername5678"),
        ..user1 // this copy the remaining elements from the other structure, Struct update syntax
    };

    let black = Color(0, 0, 0);

}

fn build_user(email: String, username: String) -> User {
    User {
        email, // because the names are the same, we dont have to duplicate code
        username,
        active: true,
        sign_in_count: 1
    }
}


// Struct is for holding multiple field in a structure
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

// Tuple Struct
struct Color(i32, i32, i32);