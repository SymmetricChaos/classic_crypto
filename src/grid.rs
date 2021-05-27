use std::fmt;

use itertools::Itertools;

#[derive(Debug,Clone)]
pub struct Grid {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<char>>
}

impl Grid {
    pub fn empty(rows: usize, cols: usize) -> Grid {
        let row = vec!['\0'; cols];
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
                row.push(symbols.next().unwrap_or('\0'));
            }
            grid.push(row)
        }
        Grid{ rows, cols, grid }
    }

    pub fn display(&self) -> String {
        let mut out = String::new();

        for r in self.grid.iter() {
            for e in r {
                if e == &'\0' {
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

    pub fn read_row_n(&self, n: usize) -> Vec<char> {
        self.grid[n].clone()
    }

    pub fn write_row_n(&mut self, n: usize, text: &str) {
        let r = text.chars().take(self.cols).collect_vec();
        for (c, p) in r.iter().zip(0..self.cols) {
            self.grid[n][p] = *c
        }
    }

    pub fn read_col_n(&self, n: usize) -> Vec<char> {
        let mut out = Vec::new();
        for row in self.grid.iter() {
            let c = row[n];
            if c != '\0' {
            }
            out.push(c)
        }
        out
    }

    pub fn write_col_n(&mut self, n: usize, text: &str) {
        let r = text.chars().take(self.rows).collect_vec();
        for (c, p) in r.iter().zip(0..self.rows) {
            self.grid[p][n] = *c
        }
    }

}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

#[test]
fn test_grid() {
    let mut g = Grid::new("thequickbrownfox", 4, 5);
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
    g.flip_diag();
    println!("{}",g);
    g.flip_diag();
    println!("{}",g);
    g.turn_counterclockwise();
    println!("{}",g);
    g.write_row_n(1, "AAA");
    g.write_row_n(3, "BBBBBB");
    g.write_col_n(1, "CCC");
    g.write_col_n(3, "DDDDD");
    println!("{}",g);
    g.turn_counterclockwise();
    println!("{}",g);
}