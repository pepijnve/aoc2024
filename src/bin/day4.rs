use aoc2024::grid;

fn xmas(grid: &grid::Grid, x: isize, y: isize, dx: isize, dy: isize) -> bool {
    let Some(c1) = grid.get(x, y) else {
        return false;
    };
    if c1 != 'X' {
        return false;
    }

    let Some(c2) = grid.get(x + dx, y + dy) else {
        return false;
    };
    if c2 != 'M' {
        return false;
    }

    let Some(c3) = grid.get(x + 2 * dx, y + 2 * dy) else {
        return false;
    };
    if c3 != 'A' {
        return false;
    }

    let Some(c4) = grid.get(x + 3 * dx, y + 3 * dy) else {
        return false;
    };
    if c4 != 'S' {
        return false;
    }

    true
}

fn mas(grid: &grid::Grid, x: isize, y: isize, dx: isize, dy: isize) -> bool {
    let Some(c1) = grid.get(x - dx, y - dy) else {
        return false;
    };
    if c1 != 'M' {
        return false;
    }

    let Some(c2) = grid.get(x, y) else {
        return false;
    };
    if c2 != 'A' {
        return false;
    }

    let Some(c3) = grid.get(x + dx, y + dy) else {
        return false;
    };
    if c3 != 'S' {
        return false;
    }

    true
}

fn main() {
    let input = aoc2024::io::read_input();
    let (a1, a2) = calculate_metrics(&input);
    println!("result 1: {}", a1);
    println!("result 2: {}", a2);
}

fn calculate_metrics(input: &str) -> (i32, i32) {
    let grid = grid::Grid::new_from_string(input);

    let mut mas_count = 0;
    let mut xmas_count = 0;
    for x in 0..grid.height() {
        for y in 0..grid.width() {
            if xmas(&grid, x, y, -1, 0) {
                xmas_count += 1;
            }
            if xmas(&grid, x, y, -1, -1) {
                xmas_count += 1;
            }
            if xmas(&grid, x, y, 0, -1) {
                xmas_count += 1;
            }
            if xmas(&grid, x, y, 1, -1) {
                xmas_count += 1;
            }
            if xmas(&grid, x, y, 1, 0) {
                xmas_count += 1;
            }
            if xmas(&grid, x, y, 1, 1) {
                xmas_count += 1;
            }
            if xmas(&grid, x, y, 0, 1) {
                xmas_count += 1;
            }
            if xmas(&grid, x, y, -1, 1) {
                xmas_count += 1;
            }

            if (mas(&grid, x, y, -1, -1) || mas(&grid, x, y, 1, 1))
                && (mas(&grid, x, y, -1, 1) || mas(&grid, x, y, 1, -1))
            {
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
    fn example() {
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
