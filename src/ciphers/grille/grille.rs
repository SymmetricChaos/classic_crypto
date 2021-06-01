use std::fmt;

use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;

use crate::alphabets::LATIN26;
use crate::grid::{Grid,BLOCKED_CELL};


/// A Grille cipher is an effective way to add nulls to a text but provides little security on its own
pub struct Grille<'a> {
    rows: usize,
    cols: usize,
    grille: Grid,
    text_length: usize,
    null_alphabet: &'a str
}

impl Grille<'_> {

    pub fn random<'a>(rows: usize, cols: usize, text_length: usize, null_alphabet: &'a str) -> Grille {

        if text_length > (rows*cols) {
            panic!("A {}x{} grille cannot contain {} symbols",rows,cols,text_length)
        }

        let mut rng = thread_rng();

        let cells = (0..(rows*cols)).collect_vec();
        let key = cells.choose_multiple(&mut rng, text_length).collect_vec();

        let mut grille = Grid::new_blocked(rows, cols);
        for k in key {
            let row = k / cols;
            let col = k % cols;
            grille.grid[row][col] = '\0';
        }

        Grille{ rows, cols, grille, text_length, null_alphabet }
    }

    pub fn random_seeded<'a>(rows: usize, cols: usize, text_length: usize, null_alphabet: &'a str, seed: u64) -> Grille {

        if text_length > (rows*cols) {
            panic!("A {}x{} grille cannot contain {} symbols",rows,cols,text_length)
        }

        let mut rng = Xoshiro256StarStar::seed_from_u64(seed);
        let cells = (0..(rows*cols)).collect_vec();
        let key = cells.choose_multiple(&mut rng, text_length).collect_vec();

        let mut grille = Grid::new_blocked(rows, cols);
        for k in key {
            let row = k / cols;
            let col = k % cols;
            grille.grid[row][col] = '\0';
        }

        Grille{ rows, cols, grille, text_length, null_alphabet }
    }

    pub fn display_grille_blank(&self) -> String {
        let mut g = self.grille.clone();
        g.replace('\u{1f}', '■');
        g.replace('\0', '□');
        g.display()
    }

    pub fn encrypt_to_grid(&self, text: &str) -> Grid {
        let tlen = text.chars().count();
        if tlen != self.text_length {
            panic!("This Grille cipher requires exactly exactly {} symbols, please add padding or encrypt block by block",
                   self.text_length)
        }

        let mut g = self.grille.clone();

        // Write in the characters to the open spaces
        g.write_rows(text);

        // Write in nulls to the BLOCKED_CELLs
        let mut rng = thread_rng();
        let null_sampler = Uniform::from(0..self.null_alphabet.chars().count());
        for row in g.grid.iter_mut() {
            for cell in row.iter_mut() {
                if *cell == BLOCKED_CELL {
                    *cell = self.null_alphabet.chars().nth(null_sampler.sample(&mut rng)).unwrap()
                }
            }
        }
        g
    }



}

impl crate::Cipher for Grille<'_> {

    fn encrypt(&self, text: &str) -> String {

        let tlen = text.chars().count();
        if tlen != self.text_length {
            panic!("This Grille cipher requires exactly exactly {} symbols, please add padding or encrypt block by block",
                   self.text_length)
        }

        let mut g = self.grille.clone();

        // Write in the characters to the open spaces
        g.write_rows(text);

        // Write in nulls to the BLOCKED_CELLs
        let mut rng = thread_rng();
        let null_sampler = Uniform::from(0..self.null_alphabet.chars().count());
        for row in g.grid.iter_mut() {
            for cell in row.iter_mut() {
                if *cell == BLOCKED_CELL {
                    *cell = self.null_alphabet.chars().nth(null_sampler.sample(&mut rng)).unwrap()
                }
            }
        }

        //println!("{}",g);

        // Read out the columns (this avoids exposing too much text unaltered)
        g.read_cols().iter().flatten().collect::<String>()

    }

    fn decrypt(&self, text: &str) -> String {
        let tlen = text.chars().count();
        if tlen != self.rows*self.cols {
            panic!("This Grille only decrypts ciphertext with exactly {} symbols",
                    self.rows*self.cols)
        }
        
        let mut g = Grid::new_empty(self.rows, self.cols);

        // Write in by columns
        g.write_cols(text);

        //println!("{}",g);

        // Read out the open spaces
        let mut out = String::with_capacity(self.text_length);
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.grille.grid[row][col] != BLOCKED_CELL {
                    out.push(g.grid[row][col])
                }
            }
        }
        out
    }
}

impl fmt::Display for Grille<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Grille Cipher\n{}", self.display_grille_blank())
    }
}

#[test]
fn test_grille_new() {
    use crate::Cipher;
    let g = Grille::random(5, 7, 17, LATIN26);
    println!("{}",g);
    let plaintext = "abcdefghijklmnopq";
    println!("{}",plaintext);
    let ciphertext = g.encrypt(plaintext);
    println!("{}",ciphertext);
    let decrypted = g.decrypt(&ciphertext);
    println!("{}",decrypted);
}