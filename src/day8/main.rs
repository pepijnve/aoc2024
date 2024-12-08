use std::collections::{HashMap, HashSet};
use std::fs;
use aoc::grid::Grid;
use aoc::vector2d::Vector2d;

fn parse_antennas(grid: &Grid) -> HashMap<char, Vec<Vector2d<isize>>> {
    let mut frequencies = HashMap::new();
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            match grid.get(x, y) {
                None => {}
                Some('.') => {}
                Some(c) => {
                    frequencies.entry(c).or_insert(Vec::new()).push(Vector2d::new(x, y));
                }
            }
        }
    }
    frequencies
}

fn point_in_grid(grid: &Grid, point: &Vector2d<isize>) -> bool {
    point.x() >= 0 && point.x() < grid.width() && point.y() >= 0 && point.y() < grid.height()
}

fn calculate_answer1(input: &str) -> i32 {
    let grid = Grid::new_from_string(input);
    let frequencies = parse_antennas(&grid);

    let mut anitnodes = HashSet::new();

    for (_, points) in &frequencies {
        for i in 0..points.len() {
            let p1 = points[i];
            for j in i + 1..points.len() {
                let p2 = points[j];
                let diff = p2 - p1;
                let anti1 = p1 - diff;
                if point_in_grid(&grid, &anti1) {
                    anitnodes.insert(anti1);
                }
                let anti2 = p2 + diff;
                if point_in_grid(&grid, &anti2) {
                    anitnodes.insert(anti2);
                }
            }
        }
    }

    anitnodes.len() as i32
}

fn calculate_answer2(input: &str) -> i32 {
    let grid = Grid::new_from_string(input);
    let frequencies = parse_antennas(&grid);

    let mut anitnodes = HashSet::new();

    for (_, points) in &frequencies {
        for i in 0..points.len() {
            let p1 = points[i];
            for j in i + 1..points.len() {
                let p2 = points[j];
                let diff = p2 - p1;
                let mut anti = p1;
                while point_in_grid(&grid, &anti) {
                    anitnodes.insert(anti);
                    anti -= diff;
                }

                let mut anti = p2;
                while point_in_grid(&grid, &anti) {
                    anitnodes.insert(anti);
                    anti += diff;
                }
            }
        }
    }

    anitnodes.len() as i32
}

fn main() {
    let input = fs::read_to_string("input/day8.txt").unwrap();
    println!("result 1: {}", calculate_answer1(&input));
    println!("result 2: {}", calculate_answer2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        let sample = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        assert_eq!(calculate_answer1(sample), 14);
        assert_eq!(calculate_answer2(sample), 34);
    }
}
