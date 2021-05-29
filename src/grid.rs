use std::fmt;

use itertools::Itertools;

const EMPTY_CELL: char = '\0';
const BLOCKED_CELL: char = '\u{1f}';

// Grid reserves 0x00 (Unicode null) to represent an unused space and 0x1F (Unicode unit seperator) to represent an unusable spaces
#[derive(Debug,Clone,Hash,PartialEq)]
pub struct Grid {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<char>>
}

impl Grid {
    pub fn new_empty(rows: usize, cols: usize) -> Grid {
        let row = vec![EMPTY_CELL; cols];
        let mut grid = Vec::<Vec<char>>::with_capacity(rows);
        for _ in 0..rows {
            grid.push(row.clone())
        }
        Grid{ rows, cols, grid }
    }

    pub fn new(text: &str, rows: usize, cols: usize) -> Grid {
        let mut symbols = text.chars();
        let mut grid = Vec::<Vec<char>>::with_capacity(rows);
        for _ in 0..rows {
            let mut row = <Vec<char>>::with_capacity(cols);
            for _ in 0..cols {
                // filling with nulls here might cause issues but wanted to use symbol not reasonably found in written text
                row.push(symbols.next().unwrap_or(EMPTY_CELL));
            }
            grid.push(row)
        }
        Grid{ rows, cols, grid }
    }

    pub fn empty(&mut self) {
        let row = vec![EMPTY_CELL; self.cols];
        for r in self.grid.iter_mut() {
            *r = row.clone()
        }
    }

    pub fn display(&self) -> String {
        let mut out = String::new();

        for r in self.grid.iter() {
            for e in r {
                if e == &EMPTY_CELL || e == &BLOCKED_CELL {
                    out.push(' ');
                } else {
                    out.push(*e);
                }
                out.push(' ')
            }
            out.push('\n')
        }
        out
    }

    // Rotate 90 degree clockwise, changes the dimensions
    pub fn turn_clockwise(&mut self) {

        let mut new_grid = Vec::<Vec<char>>::with_capacity(self.cols);

        for n in 0..self.cols {
            let mut cells = self.read_col_n(n);
            cells.reverse();
            new_grid.push(cells.clone());
        }

        let r = self.rows;
        self.rows = self.cols;
        self.cols = r;
        self.grid = new_grid;

    }

    // Rotate 90 degree counterclockwise, changes the dimensions
    pub fn turn_counterclockwise(&mut self) {
        let mut new_grid = Vec::<Vec<char>>::with_capacity(self.cols);

        for n in (0..self.cols).rev() {
            let cells = self.read_col_n(n);
            new_grid.push(cells.clone());
        }

        let r = self.rows;
        self.rows = self.cols;
        self.cols = r;
        self.grid = new_grid;
    }

    // Flip across the diagonal line from top left to bottom right, changes the dimensions
    pub fn flip_diag(&mut self) {
        let mut new_grid = Vec::<Vec<char>>::with_capacity(self.cols);

        for n in 0..self.cols {
            let cells = self.read_col_n(n);
            new_grid.push(cells.clone());
        }

        let r = self.rows;
        self.rows = self.cols;
        self.cols = r;
        self.grid = new_grid;
    }

    // Flip across the anti-diagonal line from top right to bottom left, changes the dimensions
    pub fn flip_antidiag(&mut self) {
        let mut new_grid = Vec::<Vec<char>>::with_capacity(self.cols);

        for n in (0..self.cols).rev() {
            let mut cells = self.read_col_n(n);
            cells.reverse();
            new_grid.push(cells.clone());
        }

        let r = self.rows;
        self.rows = self.cols;
        self.cols = r;
        self.grid = new_grid;
    }

    // Flip across the horizontal axis
    pub fn flip_horizontal(&mut self) {
        for (n, row) in self.read_rows().iter().rev().enumerate() {
            let r: String = row.iter().collect();
            self.overwrite_row_n(n, &r)
        }
    }

    pub fn flip_vertical(&mut self) {
        for (n, col) in self.read_cols().iter().rev().enumerate() {
            let c: String = col.iter().collect();
            self.overwrite_col_n(n, &c)
        }
    }


    pub fn read_row_n(&self, n: usize) -> Vec<char> {
        self.grid[n].clone()
    }

    pub fn read_col_n(&self, n: usize) -> Vec<char> {
        let mut out = Vec::with_capacity(self.rows);
        for row in self.grid.iter() {
            let c = row[n];
            out.push(c)
        }
        out
    }

    pub fn read_rows(&self) -> Vec<Vec<char>> {
        self.grid.clone()
    }

    pub fn read_cols(&self) -> Vec<Vec<char>> {
        let mut out_grid = Vec::<Vec<char>>::with_capacity(self.cols);
        for n in 0..self.cols {
            out_grid.push( self.read_col_n(n) )
        }
        out_grid
    }

    // Write as many characters as possible to row n from left to right, skipping any BLOCKED_CELL
    pub fn write_row_n(&mut self, n: usize, text: &str) {
        let mut new_text = text.chars();
        for c in self.grid[n].iter_mut() {
            if c != &BLOCKED_CELL {
                *c = new_text.next().unwrap_or(*c)
            }
        }
    }

    // Write as many characters as possible to row n from left to right, overwriting every cell
    pub fn overwrite_row_n(&mut self, n: usize, text: &str) {
        let mut new_text = text.chars();
        for c in self.grid[n].iter_mut() {
            *c = new_text.next().unwrap_or(*c)
        }
    }

    // Write as many characters as possible to column n from top to bottom, skipping any BLOCKED_CELL
    pub fn write_col_n(&mut self, n: usize, text: &str) {
        let mut new_text = text.chars();
        for p in 0..self.rows {
            let cur = self.grid[p][n];
            if cur != BLOCKED_CELL {
                self.grid[p][n] = new_text.next().unwrap_or(cur)
            }
        }
    }

    // Write as many characters as possible to column n from top to bottom, overwriting every cell
    pub fn overwrite_col_n(&mut self, n: usize, text: &str) {
        let mut new_text = text.chars();
        for p in 0..self.rows {
            let cur = self.grid[p][n];
            self.grid[p][n] = new_text.next().unwrap_or(cur)
        }
    }

    // Set the provided cell to BLOCKED_CELL
    pub fn block_cell(&mut self, row: usize, col: usize) {
        self.grid[row][col] = BLOCKED_CELL;
    }

    // Set the provided cell to EMPTY_CELL
    pub fn empty_cell(&mut self, row: usize, col: usize) {
        self.grid[row][col] = EMPTY_CELL;
    }

    // Replace old_char with new_char across the entire grid
    pub fn replace(&mut self, old_char: char, new_char: char) {
        for row in self.grid.iter_mut() {
            for cell in row.iter_mut() {
                if cell == &old_char {
                    *cell = new_char
                }
            }

        }
    }

}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

#[test]
fn test_grid_rotation() {
    let mut g = Grid::new("theq\u{1f}uick\u{1f}brow\u{1f}nfox\0", 5, 6);
    println!("{:?}",g.grid);
    println!("{}",g);
    g.turn_clockwise();
    println!("{}",g);
    g.turn_clockwise();
    println!("{}",g);
    g.turn_clockwise();
    println!("{}",g);
    g.turn_clockwise();
    println!("{}",g);
    g.turn_counterclockwise();
    println!("{}",g);
    g.turn_counterclockwise();
    println!("{}",g);
    g.turn_counterclockwise();
    println!("{}",g);
    g.turn_counterclockwise();
    println!("{}",g);
}


#[test]
fn test_grid_mirror() {
    let mut g = Grid::new("theq\u{1f}uick\u{1f}brow\u{1f}nfox", 5, 6);
    let normal_form = g.clone();

    println!("original grid");
    println!("{}",g);

    println!("diagonal flip");
    g.flip_diag();
    println!("{}",g);
    g.flip_diag();
    assert_eq!(g,normal_form);

    println!("anti-diagonal flip");
    g.flip_antidiag();
    println!("{}",g);
    g.flip_antidiag();
    assert_eq!(g,normal_form);

    println!("horizontal flip");
    g.flip_horizontal();
    println!("{}",g);
    g.flip_horizontal();
    assert_eq!(g,normal_form);

    println!("vertical flip");
    g.flip_vertical();
    println!("{}",g);
    g.flip_vertical();
    assert_eq!(g,normal_form);
}
    
#[test]
fn test_grid_write() {
    let mut g = Grid::new("theq\u{1f}uick\u{1f}brow\u{1f}nfox", 5, 6);
    println!("{:?}",g.grid);
    println!("{}",g);
    g.write_row_n(1, "AA");
    println!("{}",g);
    g.write_row_n(3, "BBBBBB");
    println!("{}",g);
    g.write_col_n(1, "CCCCCC");
    println!("{}",g);
    g.write_col_n(3, "DDD");
    println!("{}",g);
    g.replace('B', '#');
    println!("{}",g);
}

#[test]
fn test_grid_empty() {
    let mut g = Grid::new("theq\u{1f}uick\u{1f}brow\u{1f}nfox", 5, 6);
    println!("{:?}",g.grid);
    g.empty();
    println!("{:?}",g.grid);
}
