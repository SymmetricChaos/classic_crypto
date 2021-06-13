use std::fmt;
use std::collections::HashMap;

use itertools::Itertools;

use super::{Rotor, M209_ROTORS, Cage};


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
    cage: Cage,
}

impl<'a> M209<'a> {
    pub fn new(pins: [&str; 6], lugs: [(usize,usize);27]) -> M209<'a> {
        // Use pins to set the wheels
        // Use lugs to set the cage
        let mut wheels = M209_ROTORS.clone();
        for (r, p) in wheels.iter_mut().zip(pins) {
            r.set_pins(p)
        }
        M209{ wheels, cage: Cage::new(lugs.to_vec()) }
    }

    // The stepping is perfectly regular, every wheel steps one position each letter
    // The encryption is done by the interaction of the lugs with the active pins
    pub fn step(&mut self) {
        for w in self.wheels.iter_mut() {
            w.step()
        }
    }

    pub fn step_n(&mut self, n: usize) {
        for _ in 0..n {
            for w in self.wheels.iter_mut() {
                w.step()
            }
        }

    }

    pub fn encrypt(&mut self, text: &str) -> String {
        let nums = text.chars().map(|x| char_to_usize(x)).collect_vec();
        let out = String::with_capacity(text.len());
        
        for n in nums {
            let sh = 0;
            for bar in self.cage.bars.iter() {

            }
            // first advance the wheels
            // then iterate over the bars of the cage
            // for each lug on the bar check if that wheel has an effective pin in the active position
            // if either does sh += 1
            // finally beaufort encrypt the n with the key sh
        }

        out
    }

    pub fn decrypt(&mut self, text: &str) -> String {
        let out = String::with_capacity(text.len());
        // logic goes here (unless M209 is reciprocal, need to check that)
        out
    }

}


impl fmt::Display for M209<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "M209\n{:?}\n{}",
            self.wheels,
            self.cage)
    }
}

#[test]
fn test_m209_step() {
    let mut m209 = M209::new(["AB","CD","EF","GH","IJ","KL"], [(1,6); 27]);
    println!("{}",m209);
}