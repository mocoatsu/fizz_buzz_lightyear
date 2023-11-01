pub fn calculate_fizz_buzz(n: u8) -> String {
    match n {
        n if n % 15 == 0 => "FizzBuzz".to_string(),
        n if n % 3 == 0 => "Fizz".to_string(),
        n if n % 5 == 0 => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

pub fn run_fizz_buzz() {
    for i in 1..=100 {
        let result = calculate_fizz_buzz(i);
        println!("{}", result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn returns_a_correct_string_by_number() {
        assert_eq!(calculate_fizz_buzz(1), "1");
        assert_eq!(calculate_fizz_buzz(3), "Fizz");
        assert_eq!(calculate_fizz_buzz(5), "Buzz");
        assert_eq!(calculate_fizz_buzz(15), "FizzBuzz");
        assert_eq!(calculate_fizz_buzz(100), "Buzz");
    }
}
