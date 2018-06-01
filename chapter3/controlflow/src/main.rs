fn main() {

    simple_condition();
    else_ifs();
    if_let_example();
    loop_example();
    while_loop();
    for_loop();
    for_range_loop();
}

fn simple_condition()  {
    let number = 3;

    // Condition must be a bool
    if number < 5 {
        println!("The condition was true");
    } else {
        println!("The condition was false");
    }

}

fn else_ifs() {
    // Multiple conditions

    let number = 6;


    // Don't do this, use Match instead
    if number % 4 == 0 {
        println!("number is divisable by 4");
    } else if number % 3 == 0 {
        println!("number is divisable by 3");
    } else if number % 2 == 0 {
        println!("number is divisable by 2");
    } else {
        println!("number is not divisable by 4, 3, or 2");
    }
}

fn if_let_example() {
    let condition = true;

    // If is an expression, that why it can be used on 
    // the right side of a let statement
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

}

fn loop_example() {
    // Use break to exit the endless loop
    loop {
        println!("again!");
        break;
    }
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("Liftoff!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
}

fn for_range_loop() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("Liftoff!");
}