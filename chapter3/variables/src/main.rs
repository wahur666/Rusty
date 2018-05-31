const MAX_POINTS: u32 = 100_000;
fn main() {
    basic_assignments();
    shadowing();
}

fn basic_assignments() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);    
    println!("The value of MAX_POINTS: {}", MAX_POINTS);
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);       

    //Type changing by shadowing
    let spaces = "    ";
    let spaces = spaces.len();

    println!("NUmber of spaces: {}", spaces);

    //Error, because cant change type because it's mutable
    
    //let mut spaces = "    ";
    //spaces = spaces.len();

}
