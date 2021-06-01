use std::fmt;

use itertools::Itertools;
use rand::thread_rng;
use rand::distributions::{Distribution, Uniform};

use crate::grid::Grid;


/// A Grille cipher is an effective way to add nulls to a text but provides little security on its own
pub struct Grille<'a> {
    dimensions: (usize,usize),
    grille_length: usize,
    text_length: usize,
    key: Vec<usize>,
    null_alphabet: &'a str
}

impl Grille<'_> {
    pub fn new<'a>(dimensions: (usize,usize), key: Vec<usize>, null_alphabet: &'a str) -> Grille {
        if key.len() > (dimensions.0 * dimensions.1)/2 {
            panic!("Key should not be more than half the total spaces in the grid")
        }
        let grille_length = dimensions.0 * dimensions.1;
        let text_length = key.len();

        Grille{ dimensions, grille_length, text_length, key, null_alphabet }
    }

    pub fn display_grille_blank(&self) -> String {
        let mut v = "#".repeat(self.grille_length).chars().collect_vec();
        for hole in self.key.iter() {
            v[*hole] = '_';
        }
        let s: String = v.iter().collect();
        let g= Grid::new(&s, self.dimensions.0, self.dimensions.1);

        g.display()
    }

    pub fn display_grille_encrypted(&self, text: &str) -> String {
        let tlen = text.chars().count();
        if tlen != self.text_length {
            panic!("This cipher requires exactly exactly {} symbols, please add padding or encrypt block by block",
                   self.text_length)
        }

        let mut v = "#".repeat(self.grille_length).chars().collect_vec();
        let mut symbols = text.chars();
        for hole in self.key.iter() {
            v[*hole] = symbols.next().unwrap();
        }
        let s: String = v.iter().collect();
        let g= Grid::new(&s, self.dimensions.0, self.dimensions.1);

        g.display()
    }

}

impl crate::Cipher for Grille<'_> {

    fn encrypt(&self, text: &str) -> String {

        let tlen = text.chars().count();
        if tlen != self.text_length {
            panic!("This cipher requires exactly exactly {} symbols, please add padding or encrypt block by block",
                   self.text_length)
        }


        // First fill with nulls
        let mut rng = thread_rng();
        let null_sampler = Uniform::from(0..self.null_alphabet.chars().count());
        let mut v = Vec::with_capacity(self.grille_length);
        for _ in 0..self.grille_length {
            v.push( self.null_alphabet.chars().nth(null_sampler.sample(&mut rng)).unwrap())
        }

        // Now read in characters
        let mut symbols = text.chars();
        for hole in self.key.iter() {
            v[*hole] = symbols.next().unwrap();
        }
        let s: String = v.iter().collect();
        let g= Grid::new(&s, self.dimensions.0, self.dimensions.1);
        
        // Read off the columns
        let mut out = String::new();
        for n in 0..self.dimensions.1 {
            let s: String = g.read_col_n(n).iter().collect();
            out.push_str(&s)
        }

        out
    }

    fn decrypt(&self, text: &str) -> String {
        let tlen = text.chars().count();
        if tlen != self.grille_length {
            panic!("This cipher requires exactly exactly {} symbols",
                   self.grille_length)
        }
        // Read in by rows
        let g= Grid::new(text, self.dimensions.1, self.dimensions.0);

        // Read off the rows
        let mut sort = String::new();
        for n in 0..self.dimensions.0 {
            let s: String = g.read_col_n(n).iter().collect();
            sort.push_str(&s)
        }

        let mut out = String::new();
        for k in self.key.iter() {
            out.push(sort.chars().nth(*k).unwrap())
        }

        out
    }
}

impl fmt::Display for Grille<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Grille Cipher\n{}", self.display_grille_blank())
    }
}