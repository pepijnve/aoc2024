use std::fs;

struct Grid {
    rows: Vec<Vec<char>>,
    width: isize,
    height: isize,
}

impl Grid {
    fn new(rows: Vec<Vec<char>>) -> Grid {
        let width = rows[0].len() as isize;
        let height = rows.len() as isize;
        Grid {
            rows,
            width,
            height,
        }
    }

    fn width(&self) -> isize {
        self.width
    }

    fn height(&self) -> isize {
        self.height
    }

    fn get(&self, x: isize, y: isize) -> Option<char> {
        if x < 0 || x >= self.width() {
            return None;
        }

        if y < 0 || y >= self.height() {
            return None;
        }

        Some(self.rows[y as usize][x as usize])
    }

    fn xmas(&self, x: isize, y: isize, dx: isize, dy: isize) -> bool {
        let Some(c1) = self.get(x, y) else {
            return false;
        };
        if c1 != 'X' {
            return false;
        }

        let Some(c2) = self.get(x + dx, y + dy) else {
            return false;
        };
        if c2 != 'M' {
            return false;
        }

        let Some(c3) = self.get(x + 2 * dx, y + 2 * dy) else {
            return false;
        };
        if c3 != 'A' {
            return false;
        }

        let Some(c4) = self.get(x + 3 * dx, y + 3 * dy) else {
            return false;
        };
        if c4 != 'S' {
            return false;
        }

        true
    }

    fn mas(&self, x: isize, y: isize, dx: isize, dy: isize) -> bool {
        let Some(c1) = self.get(x - dx, y - dy) else {
            return false;
        };
        if c1 != 'M' {
            return false;
        }

        let Some(c2) = self.get(x, y) else {
            return false;
        };
        if c2 != 'A' {
            return false;
        }

        let Some(c3) = self.get(x + dx, y + dy) else {
            return false;
        };
        if c3 != 'S' {
            return false;
        }

        true
    }
}

fn main() {
    let input = fs::read_to_string("input/day4_1.txt").unwrap();
    let (xmas, mas) = calculate_metrics(&input);
    println!("result 1: {}", xmas);
    println!("result 2: {}", mas);
}

fn calculate_metrics(input: &str) -> (i32, i32) {
    let rows = input
        .split('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let grid = Grid::new(rows);

    let mut mas_count = 0;
    let mut xmas_count = 0;
    for x in 0..grid.height {
        for y in 0..grid.width {
            if grid.xmas(x, y, -1, 0) {
                xmas_count += 1;
            }
            if grid.xmas(x, y, -1, -1) {
                xmas_count += 1;
            }
            if grid.xmas(x, y, 0, -1) {
                xmas_count += 1;
            }
            if grid.xmas(x, y, 1, -1) {
                xmas_count += 1;
            }
            if grid.xmas(x, y, 1, 0) {
                xmas_count += 1;
            }
            if grid.xmas(x, y, 1, 1) {
                xmas_count += 1;
            }
            if grid.xmas(x, y, 0, 1) {
                xmas_count += 1;
            }
            if grid.xmas(x, y, -1, 1) {
                xmas_count += 1;
            }

            if (grid.mas(x, y, -1, -1) || grid.mas(x, y, 1, 1)) && (grid.mas(x, y, -1, 1) || grid.mas(x, y, 1, -1)) {
                mas_count += 1;
            }
        }
    }


    (xmas_count, mas_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        let result = calculate_metrics(
            "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        assert_eq!(result, (18, 9));
    }
}
