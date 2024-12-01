use std::fs;

fn main() {
    let input = fs::read_to_string("input/day1_1.txt").unwrap();

    let (left, right) = parse_input(input);
    let (distance, similarity) = calculate_metrics(&left, &right);

    println!("distance: {}", distance);
    println!("similarity: {}", similarity);
}

fn calculate_metrics(l: &Vec<i32>, r: &Vec<i32>) -> (i32, i32) {
    let mut left = l.clone();
    left.sort();

    let mut right = r.clone();
    right.sort();

    let mut distance = 0;
    left.iter().zip(right.iter()).for_each(|(a, b)| {
        distance += i32::abs(a - b);
    });

    let mut similarity = 0;
    let mut left_iter = left.iter().peekable();
    let mut right_iter = right.iter().peekable();
    loop {
        let a = match left_iter.peek() {
            Some(a) => **a,
            None => break,
        };
        let b = match right_iter.peek() {
            Some(b) => **b,
            None => break,
        };

        if a < b {
            left_iter.next();
        } else if a > b {
            right_iter.next();
        } else {
            let mut count_a = 0;
            while let Some(next_a) = left_iter.peek() {
                if a == **next_a {
                    count_a += 1;
                    left_iter.next();
                } else {
                    break;
                }
            }

            let mut count_b = 0;
            while let Some(next_b) = right_iter.peek() {
                if b == **next_b {
                    count_b += 1;
                    right_iter.next();
                } else {
                    break;
                }
            }

            similarity += a * count_a * count_b;
        }
    }
    (distance, similarity)
}

fn parse_input(input: String) -> (Vec<i32>, Vec<i32>) {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut left = Vec::new();
    let mut right = Vec::new();

    lines.iter().for_each(|line| {
        let left_right = line.split_whitespace().collect::<Vec<&str>>();
        left.push(left_right[0].parse::<i32>().unwrap());
        right.push(left_right[1].parse::<i32>().unwrap());
    });
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        let input = "\
3   4
4   3
2   5
1   3
3   9
3   3";
        let (left, right) = parse_input(input.to_string());
        assert_eq!(left.len(), 6);
        assert_eq!(right.len(), 6);
        let (distance, similarity) = calculate_metrics(&left, &right);
        assert_eq!(distance, 11);
        assert_eq!(similarity, 31);
    }
}
