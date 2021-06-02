use std::fmt;

use crate::grid::{BLOCKED_CELL, Grid};

pub struct TurningGrille {
    grille: Grid,
    text_length: usize,
    width: usize,
}

impl TurningGrille {
    pub fn new(subgrille_width: usize, key: Vec<usize>) -> TurningGrille {
        if subgrille_width % 2 != 0 {
            panic!("Subgrille width must be even")
        }
        if key.len() != subgrille_width*subgrille_width {
            panic!("Turning Grille Cipher requires exactly every position in the subgrille to be included")
        }
        // Check that key is unique naturals from 0 to the square of subgrille_width

        let grille_width = subgrille_width*2;

        // Create a new grille and block every space
        let mut grille = Grid::new_empty(grille_width, grille_width);
        grille.replace('\0','\u{1f}');

        for (pos, n) in key.iter().enumerate() {
            if pos % subgrille_width == 0 {
                grille.turn_counterclockwise()
            }
            let col = n % subgrille_width;
            let row = n / subgrille_width;
            if col >= subgrille_width {
                panic!("Key value outside of subgrille")
            }
            grille.empty_cell(row,col);
        }
        grille.turn_counterclockwise();

        let text_length = grille_width * grille_width;
        TurningGrille{ grille, text_length, width: grille_width }
    }


    pub fn display_grille_blank(&self) -> String {
        let mut g = self.grille.clone();
        g.replace('\u{1f}', '■');
        g.replace('\0', '□');
        g.display()
    }
}

impl crate::Cipher for TurningGrille {

    fn encrypt(&self, text: &str) -> String {
        if text.chars().count() != self.text_length {
            panic!("Text must have exactly {} characters, please add padding", self.text_length)
        }
        let mut crypto_grid = self.grille.clone();
        let section = self.text_length/4;

        let mut text_grid = Grid::new_empty(self.width, self.width);

        for i in 0..4 {
            let lo = i*section;
            let hi = lo+section;
            let mut snip = text[lo..hi].chars();
            for row in 0..self.width {
                for col in 0..self.width {
                    if crypto_grid[row][col] != BLOCKED_CELL {
                        text_grid[row][col] = snip.next().unwrap()
                    }
                }
            }
            crypto_grid.turn_clockwise();
        }
        text_grid.read_cols().iter().flatten().collect::<String>()
    }

    fn decrypt(&self, text: &str) -> String {
        let mut text_grid = Grid::new_empty(self.width, self.width);
        text_grid.write_cols(text);
        let mut crypto_grid = self.grille.clone();

        let mut out = String::with_capacity(text.len());
        for _ in 0..4 {
            for row in 0..self.width {
                for col in 0..self.width {
                    if crypto_grid[row][col] != BLOCKED_CELL {
                        out.push(text_grid[row][col])
                    }
                }
            }

            crypto_grid.turn_clockwise();
        }
        out
    }
}
 
impl fmt::Display for TurningGrille {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Turning Grille\n{}",self.display_grille_blank())
    }
}