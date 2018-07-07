fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let res = largest_i32(&number_list);
    println!("The largest number is {}", res);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let res = largest_i32(&number_list);
    println!("The largest number is {}", res);

    let char_list = vec!['y', 'm', 'z', 'q'];

    let res = largest_char(&char_list);
    println!("The largest number is {}", res);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let res = largest(&number_list);
    println!("The largest number is {}", res);

    let char_list = vec!['y', 'm', 'z', 'q'];

    let res = largest(&char_list);
    println!("The largest number is {}", res);

    let integer_point = Point{ x:5, y:10 };
    let float_point = Point{ x:1.0, y:4.0 };

}
