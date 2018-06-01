fn main() {
    println!("Hello, world!");
    
    snek_case();
    function_with_param(5);
    function_with_params(5, 6);
    exp_example();
    let x = five();
    println!("The value of x is: {}", x);
    
    let x = six();
    println!("The value of x is: {}", x);
}

fn snek_case() {
    println!("Another function");
}

// Function params always require type annotation
fn function_with_param(x: i32) {
    println!("The value of x is: {}", x);
}

fn function_with_params(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn exp_example(){
    // This is an expression example
    let y = {
        let x = 3;
        /* See here is no semicolon on the end, that is
        why this is an expression, it returns the value 
        of x + 1. If you put semicolon on the end, this
        will be a statement and does not return anything.*/ 
        x + 1
    };

    println!("The value of y is: {:?}", y);
}

// If a function returns a value, there should be a type annotation
fn five() -> i32 {
    5
}


// Use return for early exit, or the last expression will be
// returned by the function by default.
fn six() -> i32 {
    return 6;
}

