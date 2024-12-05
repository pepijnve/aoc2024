use std::cmp::Ordering::{Equal, Less};
use std::collections::HashMap;
use std::fs;

fn parse_input(input: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut lines = input.lines();

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let parts = line.split("|").collect::<Vec<&str>>();
        let page1 = parts[0].parse::<i32>().unwrap();
        let page2 = parts[1].parse::<i32>().unwrap();
        if let Some(pages) = rules.get_mut(&page1) {
            pages.push(page2);
        } else {
            rules.insert(page1, vec![page2]);
        }
    }
    rules.values_mut().for_each(|v| v.sort());

    let updates = lines
        .map(|line| {
            line.split(",")
                .map(|page| page.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    (rules, updates)
}

fn sort_by_rules(rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> Vec<i32> {
    let mut fixed = update.clone();
    fixed.sort_by(|a, b| {
        let Some(rule) = rules.get(&a) else {
            return Equal;
        };
        return if rule.contains(&b) {
            Less
        } else {
            Equal
        }
    });
    fixed
}

fn calculate_answer1(input: &str) -> i32 {
    let (rules, updates) = parse_input(input);

    let mut result = 0;

    for update in updates {
        let sorted = sort_by_rules(&rules, &update);
        if sorted.eq(&update) {
            result += update[update.len() / 2];
        }
    }

    result
}

fn calculate_answer2(input: &str) -> i32 {
    let (rules, updates) = parse_input(input);

    let mut result = 0;

    for update in updates {
        let sorted = sort_by_rules(&rules, &update);
        if !sorted.eq(&update) {
            result += sorted[sorted.len() / 2];
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("input/day5_1.txt").unwrap();
    println!("result 1: {}", calculate_answer1(&input));
    println!("result 2: {}", calculate_answer2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        let sample = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(calculate_answer1(sample), 143);
        assert_eq!(calculate_answer2(sample), 123);
    }
}
