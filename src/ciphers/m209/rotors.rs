use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};

#[derive(Clone,Debug,Copy)]
pub struct Rotor<'a> {
    alphabet: &'a str,
    pins: &'a str,
    position: usize,
    alphabet_len: usize,
}

impl Rotor<'_> {
    pub fn new(alphabet: &str) -> Rotor {
        Rotor{ alphabet, pins: "", position: 0, alphabet_len: alphabet.chars().count() }
    }

    pub fn step(&mut self) {
        self.position = (self.position + 1) % self.alphabet_len
    }

    pub fn set_pins(&mut self, pins: &str) {
        self.pins = pins
    }

    pub fn get_pins(&mut self, pins: &str) -> String {
        self.pins.to_string()
    }

    pub fn set_position(&mut self, n: usize) {
        self.position = n
    }

    pub fn get_position(&self) -> usize {
        self.position
    }


}

// This could be simplified since all the real rotors used ASCII characters but this library tries to work with Unicode as much as possible
impl fmt::Display for Rotor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "M209 Rotor")
    }
}


//The rotor alphabets all have coprime lengths
lazy_static! {
    pub static ref M209_ROTORS: [Rotor<'static>; 6] = {
        [Rotor::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        Rotor::new("ABCDEFGHIJKLMNOPQRSTUVXYZ"),
        Rotor::new("ABCDEFGHIJKLMNOPQRSTUVX"),
        Rotor::new("ABCDEFGHIJKLMNOPQRSTU"),
        Rotor::new("ABCDEFGHIJKLMNOPQRS"),
        Rotor::new("ABCDEFGHIJKLMNOPQ"),
        ]
    };
}