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

    pub fn display(&self) {
        for r in self.grid.iter() {
            for e in r {
                print!("{} ",e);
            }
            println!("");
        }
    }

    pub fn transpose(&mut self) {

    }

    pub fn turn_clockwise(&mut self) {

    }

    pub fn turn_counterclockwise(&mut self) {
        
    }

    pub fn read_by_rows(&self) -> String {
        let mut out = String::new();
        for row in self.grid.iter() {
            let mut s: String = row.iter().collect();
            s = s.replace('\0', "");
            out.push_str(&s)
        }
        out
    }

    pub fn read_row_n(&self, n: usize) -> String {
        let mut out: String = self.grid[n].iter().collect();
        out = out.replace('\0', "");
        out
    }

    pub fn read_col_n(&self, n: usize) -> String {
        let mut out = String::new();
        for row in self.grid.iter() {
            let c = row[n];
            if c != '\0' {
            }
            out.push(c)
        }
        out
    }

    pub fn read_by_cols(&self) -> String {
        let mut out = String::new();
        for c in 0..self.cols {
            out.push_str(&self.read_col_n(c))
        }
        out
    }

}

#[test]
fn test_grid() {
    let g = Grid::new("thequickbrownfox", 4, 5);
    println!("{:?}",g.grid);
    g.display();
    println!("{}",g.read_by_rows());
    println!("{}",g.read_row_n(1));
    println!("{}",g.read_by_cols());
    println!("{}",g.read_col_n(1));
}