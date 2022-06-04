#[cfg(test)]
mod tests {
    
    use super::*;
    use rust111::{Rectangle, Guess};

    #[test]
    fn canary_test() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // checking results with the assert! Macro
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7};
        let smaller = Rectangle { width: 4, height: 3};

        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));

    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contain_name() {
        let result = greeting("Steve");
        assert!(
            result.contains("Steve"),
            "value was {}",
            result
        )
    }

    #[test]
    fn greeting_not_contain_name() {
        let result = greeting("Steve");
        // adding custom failed message
        assert!(
            result.contains("Steve"),
            "Greeting did not contain name, value was {}",
            result
        );
    }

    // checking for panics with should_panic
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(333);
    }

    // use result<T, E> in tests
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two != 4"))
        }
    }
}

fn add_two(n: i32) -> i32 {
    n + 2
}

#[allow(unused)]
fn greeting(name: &str) -> String {
    format!("Hello")
}
