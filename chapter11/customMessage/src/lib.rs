pub fn greeting (name: &str) -> String {
    format!("Hellow {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less or equal to 100, got {}.", value);
        }

        Guess {
            value
        }
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name value was `{}`", result
        );
    }    

    #[test]
    #[should_panic(expected = "Guess value must be less or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}