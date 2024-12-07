pub struct Grid {
    rows: Vec<Vec<char>>,
    width: isize,
    height: isize,
}

impl Clone for Grid {
    fn clone(&self) -> Grid {
        Grid::new(self.rows.iter().map( |r| r.clone()).collect())
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
        Grid {
            rows,
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
        if x < 0 || x >= self.width() {
            return None;
        }

        if y < 0 || y >= self.height() {
            return None;
        }
        Some(self.rows[y as usize][x as usize])
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
        if x < 0 || x >= self.width() {
            return;
        }

        if y < 0 || y >= self.height() {
            return;
        }

        self.rows[y as usize][x as usize] = c
    }
}
