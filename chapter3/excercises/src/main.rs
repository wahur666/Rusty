fn main() {
    let x = fibonnaci(10);
    println!("The 10th element of the fibonacci: {}", x);

    let x = 99.;
    println!("Is the methods correct? {}", to_celsius(to_farenheit(x)) == x);

}

fn to_celsius(farenheit: f32) -> f32 {
    (farenheit - 32.) * 5./9.
}

fn to_farenheit(celsius: f32) -> f32 {
    celsius * 9 as f32 /5 as f32 + 32 as f32
}

fn fibonnaci(n: i32) -> i32 {
    if n == 0 || n == 1 {
        n
    } else {
        fibonnaci(n - 1) + fibonnaci(n - 2)
    }
}
