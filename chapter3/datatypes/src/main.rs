fn main() {

    //When multiple types can be inferred, you have to specify the type or get an error
    let guess: u128 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    // Integer types

    // Signed integers start with an i
    // Unsigned integers starts with an u

    //Types
    // i8, u8
    // i16, u16
    // i32, u32
    // i64, u64
    // i128, u128
    // isize, usize <- This means the variable will be the size of the architecture which running on. 
    // 64bit -> i64/u64, 32bit -> i32/u32

    // Floating-Points Types
    // f32, f64

    let x = 2.0; // f64
    let y : f32 = 3.0; // f32

    // Numeric Operations

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // Boolean type

    println!("{}", true && false);

    let t = true;

    let f: bool = false; // with explicit type annotation

    // Character type

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    // Compound Types
    
    // Compound types can group multiple values into one type
    // Tuples (x, y, z)
    // Arrays [1, 2, 3]

    

}
