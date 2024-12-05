use std::fs;

fn calculate_answer1(input: &str) -> i32 {
    0
}

fn calculate_answer2(input: &str) -> i32 {
    0
}

fn main() {
    let input = fs::read_to_string("input/day_1.txt").unwrap();
    println!("result 1: {}", calculate_answer1(&input));
    println!("result 2: {}", calculate_answer2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        let sample = "\
";

        assert_eq!(calculate_answer1(sample), 0);
        assert_eq!(calculate_answer1(sample), 0);
    }
}
