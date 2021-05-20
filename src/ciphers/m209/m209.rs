use std::fmt;
use std::collections::HashMap;

use super::{Rotor, M209_ROTORS, Drum};


// The M209 was a reciprocal cipher

fn char_to_usize(c: char) -> usize {
    (c as u8 as usize) - 65
}

fn usize_to_char(n: usize) -> char {
    (n + 65) as u8 as char
}

/*
Pin Settings
*/

/*
Lug Settings
*/



#[derive(Clone,Debug)]
pub struct M209<'a> {
    wheels: [Rotor<'a>; 6],
    drum: Drum,
}

impl<'a> M209<'a> {
    pub fn new(pins: [&str; 6], lugs: [(usize,usize);27]) -> M209<'a> {
        // Use pins to set the wheels
        // Use lugs to set the drum
        M209{ wheels: M209_ROTORS.clone(), drum: Drum::new(vec![(1,6)]) }
    }

    pub fn encrypt(&self, text: &str) -> String {
        let out = String::with_capacity(text.len());
        // logic goes here
        out
    }

    pub fn decrypt(&self, text: &str) -> String {
        let out = String::with_capacity(text.len());
        // logic goes here (unless M209 is reciprocal, need to check that)
        out
    }

}


impl fmt::Display for M209<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "M209\n{:?}\n{:?}",
            self.wheels,
            self.drum)
    }
}