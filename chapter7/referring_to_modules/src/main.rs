pub mod a{
    pub mod series {
        pub mod of {
            pub fn nested_modules() {

            }
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// use a::series::of;
use a::series::of::nested_modules;

//Multiple use
use TrafficLight::{Red, Yellow};

// Bring all
use TrafficLight::*;

fn main() {
    // of::nested_modules();
    nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
