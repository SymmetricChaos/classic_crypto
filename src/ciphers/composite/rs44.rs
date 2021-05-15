use std::{cell::Cell, fmt};

const RS44_DIGRAPHS: [&'static str; 25] = ["aa", "ba", "ca", "da", "ea", "ab", "bb", "cb", "db", "eb", "ac", "bc", "cc", "dc", "ec", "ad", "bd", "cd", "dd", "de", "ae", "be", "ce", "de", "ee"];

/// The Rasterschlüssel 44 (RS44) Cipher is a sophisticated grille cipher
pub struct RS44 {
    column_nums: [usize; 25],
    column_pairs: [&'static str; 25],
    row_pairs: [&'static str; 24],
    dimensions: (usize,usize), // there are always 25 columns and 24 rows
    spaces: [usize; 240], // exactly ten spaces are left open in each row 
    start: Cell<(usize,usize)>,
}


impl RS44 {
    pub fn new(column_nums: [usize; 25], column_pairs: [&'static str; 25], row_pairs: [&'static str; 24], spaces: [usize; 240]) -> RS44 {
        RS44{ column_nums, column_pairs, row_pairs, dimensions: (25,24), spaces, start: Cell::new((0,0)) }
    }

/*     fn time() -> String {

    } */

    fn create_empty_grid<'b>(&self) -> Vec<Vec<char>> {
        let mut grid = Vec::new();
        for _ in 0..self.dimensions.0 {
            let mut v = Vec::with_capacity(self.dimensions.1);
            for _ in 0..self.dimensions.1 {
                v.push('\0')
            }
            grid.push(v)
        }
        grid
    }

    fn set_start(&self, cell: (usize,usize)) {
        self.start.set(cell)
    }
}

impl crate::auxiliary::Cipher for RS44 {
    
    fn encrypt(&self, text: &str) -> String {

    }

    fn decrypt(&self, text: &str) -> String {

    }

}

impl fmt::Display for RS44 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RS44 Composite Cipher")
    }
}
