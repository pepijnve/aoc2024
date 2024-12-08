fn main() {
    let input = aoc2024::io::read_input();

    let reports = parse_input(input);
    let (safe, safe_with_dampening) = calculate_metrics(&reports);

    println!("safe: {}", safe);
    println!("safe_with_dampening: {}", safe_with_dampening);
}

fn calculate_metrics(reports: &Vec<Vec<i32>>) -> (usize, usize) {
    (
        reports.iter().filter(|report| is_safe(report)).count(),
        reports
            .iter()
            .filter(|report| is_safe_with_dampening(report))
            .count(),
    )
}

fn is_safe_with_dampening(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut dampened = report.clone();
        dampened.remove(i);
        if is_safe(&dampened) {
            return true;
        }
    }

    false
}

fn is_safe(report: &Vec<i32>) -> bool {
    let sign = i32::signum(report[1] - report[0]);
    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        if i32::signum(diff) != sign {
            return false;
        }

        let abs_diff = i32::abs(diff);
        if abs_diff < 1 || abs_diff > 3 {
            return false;
        }
    }
    true
}

fn parse_input(input: String) -> Vec<Vec<i32>> {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut reports = Vec::new();

    lines.iter().for_each(|line| {
        let report = line
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        reports.push(report);
    });

    reports
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let reports = parse_input(input.to_string());
        assert_eq!(reports.len(), 6);
        let (safe, safe_with_dampening) = calculate_metrics(&reports);
        assert_eq!(safe, 2);
        assert_eq!(safe_with_dampening, 4);
    }
}
