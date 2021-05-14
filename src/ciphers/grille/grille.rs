use std::fmt;

use rand::thread_rng;
use rand::distributions::{Distribution, Uniform};



pub struct Grille<'a> {
    dimensions: (usize,usize),
    grille_length: usize,
    null_length: usize,
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
        let null_length = grille_length - text_length;
        Grille{ dimensions, grille_length, text_length, null_length, key, null_alphabet }
    }

    pub fn display_grille_blank(&self) -> String {
        let mut out = String::with_capacity(self.grille_length);
        let mut ctr = 0;
        for _x in 0..self.dimensions.0 {
            for _y in 0..self.dimensions.1 {
                match self.key.contains(&ctr) {
                    true => out.push('_'),
                    false => out.push('#')
                }
                ctr += 1;
                out.push(' ')
            }
            out.push('\n')
        }
        out
    }

    pub fn display_grille_encrypted(&self, text: &str) -> String {
        let tlen = text.chars().count();
        if tlen != self.text_length {
            panic!("This cipher requires exactly exactly {} symbols, please add padding or encrypt block by block",
                   self.text_length)
        }
        let mut rng = thread_rng();
        let mut symbols = text.chars();
        let nulls = {
            let null_sampler = Uniform::from(0..self.null_alphabet.chars().count());
            let mut v = Vec::new();
            for _ in 0..self.null_length {
                v.push(self.null_alphabet.chars().nth(null_sampler.sample(&mut rng)).unwrap())
            }
            v
        };
        let mut null_iter = nulls.iter();
        let mut out = String::with_capacity(self.grille_length);
        let mut ctr = 0;
        for _x in 0..self.dimensions.0 {
            for _y in 0..self.dimensions.1 {
                match self.key.contains(&ctr) {
                    true => out.push(symbols.next().unwrap() ),
                    false => out.push( *null_iter.next().unwrap() )
                }
                ctr += 1;
                out.push(' ')
            }
            out.push('\n')
        }

        out
    }

}

impl crate::Cipher for Grille<'_> {

    fn encrypt(&self, text: &str) -> String {
        let tlen = text.chars().count();
        if tlen != self.text_length {
            panic!("This cipher requires exactly exactly {} symbols, please add padding or encrypt block by block",
                   self.text_length)
        }
        let mut rng = thread_rng();
        let mut symbols = text.chars();
        let null_sampler = Uniform::from(0..self.null_alphabet.chars().count());
        let mut out = String::with_capacity(self.grille_length);
        for i in 0..self.grille_length {
            match self.key.contains(&i) {
                true => out.push(symbols.next().unwrap()),
                false => out.push( self.null_alphabet.chars().nth(null_sampler.sample(&mut rng)).unwrap())
            }
        }

        out
    }

    fn decrypt(&self, text: &str) -> String {
        let tlen = text.chars().count();
        if tlen != self.grille_length {
            panic!("This cipher requires exactly exactly {} symbols, please add padding or encrypt block by block",
                   self.grille_length)
        }
        let mut out = String::with_capacity(self.text_length);
        for i in self.key.iter() {
            out.push(text.chars().nth(*i).unwrap())
        }
        out
    }
}

impl fmt::Display for Grille<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Grille Cipher")
    }
}