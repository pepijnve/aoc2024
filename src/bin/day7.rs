fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(":").collect::<Vec<&str>>();
            let test_value = parts[0].parse::<u64>().unwrap();
            let operands = parts[1]
                .split_whitespace()
                .map(|o| o.parse::<u64>().unwrap())
                .collect();
            (test_value, operands)
        })
        .collect()
}

fn can_evaluate_to_recursive(
    test_value: u64,
    allow_concat: bool,
    accumulator: u64,
    operands: &[u64],
) -> bool {
    if accumulator > test_value {
        return false;
    }

    if operands.len() == 0 {
        return accumulator == test_value;
    }

    let remainder = if operands.len() > 1 {
        &operands[1..]
    } else {
        &[]
    };

    let operand = operands[0];
    can_evaluate_to_recursive(test_value, allow_concat, accumulator + operand, remainder)
        || can_evaluate_to_recursive(test_value, allow_concat, accumulator * operand, remainder)
        || (allow_concat
            && can_evaluate_to_recursive(
                test_value,
                allow_concat,
                format!("{}{}", accumulator, operand).parse().unwrap(),
                remainder,
            ))
}

fn can_evaluate_to(test_value: u64, allow_concat: bool, operands: &[u64]) -> bool {
    match operands.len() {
        0 => false,
        1 => test_value == operands[0],
        _ => can_evaluate_to_recursive(test_value, allow_concat, operands[0], &operands[1..]),
    }
}

fn calculate_answer1(input: &str) -> u64 {
    let input_lines = parse_input(input);
    input_lines
        .iter()
        .filter(|(test_value, operands)| can_evaluate_to(*test_value, false, &operands))
        .map(|(test_value, _)| test_value)
        .sum()
}

fn calculate_answer2(input: &str) -> u64 {
    let input_lines = parse_input(input);
    input_lines
        .iter()
        .filter(|(test_value, operands)| can_evaluate_to(*test_value, true, &operands))
        .map(|(test_value, _)| test_value)
        .sum()
}

fn main() {
    let input = aoc2024::io::read_input();
    println!("result 1: {}", calculate_answer1(&input));
    println!("result 2: {}", calculate_answer2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let sample = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        assert_eq!(calculate_answer1(sample), 3749);
        assert_eq!(calculate_answer2(sample), 11387);
    }
}
