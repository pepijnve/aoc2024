use std::fs;
use aoc::grid;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}
fn distinct_steps(grid: &mut grid::Grid) -> Option<i32> {
    let mut distinct_steps = 1;

    let (mut x, mut y) = grid.find('^').unwrap();

    grid.set(x, y, '^');
    let mut direction = Direction::Up;
    loop {
        let (dx, dy, dir_char): (isize, isize, char) = match direction {
            Direction::Up => (0, -1, '^'),
            Direction::Right => (1, 0, '>'),
            Direction::Down => (0, 1, 'v'),
            Direction::Left => (-1, 0, '<'),
        };

        match grid.get(x + dx, y + dy) {
            Some('#') => {
                direction = match direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                };
                continue
            }
            Some('.') => {
                grid.set(x + dx, y + dy, dir_char);
                distinct_steps += 1;
            }
            Some(c) => {
                if c == dir_char {
                    return None
                }
            }
            None => break,
        }
        x += dx;
        y += dy;
    }

    Some(distinct_steps)
}

fn calculate_answer1(input: &str) -> i32 {
    let mut grid = grid::Grid::new_from_string(input);
    distinct_steps(&mut grid).unwrap()
}

fn calculate_answer2(input: &str) -> i32 {
    let grid = grid::Grid::new_from_string(input);

    let mut possible_loops_positions = 0;

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if let Some(char) = grid.get(x, y) {
                if char == '.' {
                    let mut with_obstacle = grid.clone();
                    with_obstacle.set(x, y, '#');
                    if let None = distinct_steps(&mut with_obstacle) {
                        possible_loops_positions += 1;
                    }
                }
            }
        }
    }

    possible_loops_positions
}

fn main() {
    let input = fs::read_to_string("input/day6.txt").unwrap();
    println!("result 1: {}", calculate_answer1(&input));
    println!("result 2: {}", calculate_answer2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        let sample = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        assert_eq!(calculate_answer1(sample), 41);
        assert_eq!(calculate_answer2(sample), 6);
    }
}
