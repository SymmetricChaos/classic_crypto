use std::fmt;

use crate::errors::CipherError;
use std::collections::{VecDeque,HashMap};
use crate::alphabet::CipherAlphabet;
use crate::modulus::*;

#[derive(Clone,Debug)]
pub struct Rotor {
    wiring: VecDeque<char>
}

impl Rotor {
    pub fn new(alphabet: &str) -> Rotor {
        let wiring: VecDeque<char> = alphabet.chars().into_iter().collect();
        Rotor{ wiring }
    }

    pub fn step(&mut self) {
        self.wiring.rotate_left(1);
    }

    pub fn swap(&mut self, character: char) -> char {

    }
}





#[derive(Clone,Debug)]
pub struct Plugboard {
    wiring: HashMap<char,char>
}

impl Plugboard {
    pub fn new(pairs: Vec<(char,char)>) -> Plugboard {
        let mut wiring = HashMap::new();
        for (p1, p2) in pairs {
            wiring.insert(p1,p2);
            wiring.insert(p2,p1);
        }
        Plugboard{ wiring }
    }

    pub fn swap(&self, character: char) -> char {
        if self.wiring.contains_key(&character) {
            self.wiring[&character]
        } else {
            character
        }
    }
}





#[derive(Clone,Debug)]
pub struct Settings {
    plugboard: Plugboard,
    rotors: Vec<Rotor>,
    ring_positions: Vec<u8>,
}

impl Settings {
    pub fn new(plugboard: Plugboard, rotors: Vec<Rotor>, ring_positions: Vec<u8>) -> Settings {
        Settings{ plugboard, rotors, ring_positions }
    }
}





pub struct Enigma {
    key: Settings,

}

impl Enigma {
    pub fn new(key: Settings) -> Enigma {
        Enigma{ key }
    }

    pub fn rotor_positions(&self, ) {

    }

/*     pub fn encode(&self, text: &str) -> Result<String,CipherError> {

    } */
}

impl fmt::Display for Enigma {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Enigma Cipher\nPlugboard: {}\nRotors: {}\nRing Positions:{}",self.key.plugboard, self.key.rotors, self.key.ring_positions)
    }
}





#[test]
fn affine() {
    let board = Plugboard::new(vec![('A','B'),('X','Z')]);

    println!("{} -> {}",'A',board.swap('A'));
    println!("{} -> {}",'B',board.swap('B'));
    println!("{} -> {}",'C',board.swap('C'));
    
}