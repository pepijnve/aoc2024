use std::fs;
use regex::Regex;
fn main() {
    let input = fs::read_to_string("input/day3_1.txt").unwrap();
    println!("result 1: {}", calculate_metrics(&input, true));
    println!("result 2: {}", calculate_metrics(&input, false));
}

fn calculate_metrics(input: &str, force_enabled: bool ) -> i32 {
    let re = Regex::new("(?<do>do\\(\\))|(?<dont>don't\\(\\))|(?<mul>mul\\((?<first>[0-9]+),(?<second>[0-9]+)\\))").unwrap();
    let iter = re.captures_iter(input);
    let mut result = 0;
    let mut enabled = true;
    for capture in iter {
        if let Some(_) = capture.name("do") {
            enabled = true;
        } else if let Some(_) = capture.name("dont") {
            enabled = force_enabled;
        } else if let Some(_) = capture.name("mul") {
            if enabled {
                let v1 = capture.name("first").unwrap().as_str().parse::<i32>().unwrap();
                let v2 = capture.name("second").unwrap().as_str().parse::<i32>().unwrap();
                result += v1 * v2;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        let result = calculate_metrics(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
            true
        );
        assert_eq!(result, 161);

        let result = calculate_metrics(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
            false
        );
        assert_eq!(result, 48);
    }
}
