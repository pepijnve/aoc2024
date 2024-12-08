fn calculate_answer1(_: &str) -> i32 {
    0
}

fn calculate_answer2(_: &str) -> i32 {
    0
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
";

        assert_eq!(calculate_answer1(sample), 0);
        assert_eq!(calculate_answer2(sample), 0);
    }
}
