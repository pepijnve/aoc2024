pub struct Grid {
    grid: Box<[char]>,
    width: isize,
    height: isize,
}

impl Clone for Grid {
    fn clone(&self) -> Grid {
        let grid = self.grid.clone();
        let width = self.width;
        let height = self.height;
        Grid {
            grid,
            width,
            height
        }
    }
}

impl Grid {
    pub fn new_from_string(input: &str) -> Grid {
        let rows = input
            .split('\n')
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        Grid::new(rows)
    }

    pub fn new(rows: Vec<Vec<char>>) -> Grid {
        let width = rows[0].len() as isize;
        let height = rows.len() as isize;
        let grid: Box<[char]> = rows.into_iter().flat_map(|r| r.into_iter()).collect();
        Grid {
            grid,
            width,
            height,
        }
    }

    pub fn width(&self) -> isize {
        self.width
    }

    pub fn height(&self) -> isize {
        self.height
    }

    pub fn get(&self, x: isize, y: isize) -> Option<char> {
        let width = self.width();
        if x < 0 || x >= width {
            return None;
        }

        if y < 0 || y >= self.height() {
            return None;
        }
        Some(self.grid[(y * width + x) as usize])
    }

    pub fn find(&self, c: char) -> Option<(isize, isize)> {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if let Some(char) = self.get(x, y) {
                    if char == c {
                        return Some((x, y));
                    }
                }
            }
        }

        None
    }

    pub fn set(&mut self, x: isize, y: isize, c: char) {
        let width = self.width();
        if x < 0 || x >= width {
            return;
        }

        if y < 0 || y >= self.height() {
            return;
        }

        self.grid[(y * width + x) as usize] = c
    }
}
