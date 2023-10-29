

struct Rule {
    divisor: i32,
    word: String,
}

fn apply_rule(number: i32, rule: &Rule) -> String {
    if is_divisible_by(number, rule.divisor) {
        rule.word.to_owned()
    } else {
        "".to_owned()
    }
}

fn is_divisible_by(number: i32, divisor: i32) -> bool {
    (number % divisor) == 0
}

fn fizzbuzz(number: i32) -> String {
    let rules: [Rule; 2] = [
        Rule { divisor: 3, word: "Fizz".to_owned() },
        Rule { divisor: 5, word: "Buzz".to_owned() }
    ];

    let result = rules.iter().fold(
        "".to_owned(),
        |acc, rule| format!("{}{}", acc, apply_rule(number, rule))
    );

    if !result.is_empty() { result } else { number.to_string() }
}

#[cfg(test)]
mod tests {
    use crate::fizzbuzz::fizzbuzz::fizzbuzz;

    #[test]
    fn should_work() {
        assert_eq!("a", "a");
    }

    #[test]
    fn should_be_a_number() {
        assert_eq!(fizzbuzz(1), "1");
    }

    #[test]
    fn should_be_another_number() {
        assert_eq!(fizzbuzz(2), "2");
    }

    #[test]
    fn should_return_fizz_if_number_divisible_for_3() {
        assert_eq!(fizzbuzz(9), "Fizz");
    }

    #[test]
    fn should_return_buzz_if_number_divisible_for_5() {
        assert_eq!(fizzbuzz(25), "Buzz");
    }

    #[test]
    fn should_return_fizzbuzz_if_number_divisible_for_15() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
    }
}