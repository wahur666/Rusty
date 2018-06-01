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

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);

    // Destructuring 
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);


    // Indexing tuple
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // Arrays [1, 2, 3]
    // Immutable, does not grow or shrink, allocates on the stack
    // Use it when the data is not changing, else use vectors

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a = [1, 2, 3, 4, 5];

    // Indexing array
    let first = a[0];
    let second = a[1];

    println!("{}", second);

    let x = a[10];

    println!("Error : {}", x);

}
