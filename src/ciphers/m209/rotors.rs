use lazy_static::lazy_static;
use std::{collections::VecDeque, fmt};

#[derive(Clone,Debug)]
pub struct Rotor {
    alphabet: VecDeque<char>,
    pins: Vec<char>,
    active: usize,
}

impl Rotor {
    pub fn new(alphabet: &str, active: usize) -> Rotor {
        let alphabet: VecDeque<char> = alphabet.chars().collect();
        Rotor{ alphabet, pins: Vec::new(), active }
    }

    pub fn step(&mut self) {
        self.alphabet.rotate_right(1)
    }

    pub fn set_pins(&mut self, pins: &str) {
        for p in pins.chars() {
            if !self.alphabet.contains(&p) {
                panic!("effective pins must be in the Rotor's alphabet")
            }
        }
        self.pins = pins.chars().collect()
    }

    pub fn get_pins(&mut self) -> Vec<char> {
        self.pins.clone()
    }

    pub fn set_active(&mut self, c: char) {
        while self.alphabet[self.active] != c {
            self.alphabet.rotate_right(1)
        }
    }

    pub fn set_display(&mut self, c: char) {
        while self.alphabet[0] != c {
            self.alphabet.rotate_right(1)
        }
    }

    pub fn get_active(&self) -> char {
        self.alphabet[self.active]
    }

    pub fn active_is_effective(&self) -> bool {
        self.pins.contains(&self.alphabet[self.active])
    }

}

// This could be simplified since all the real rotors used ASCII characters but this library tries to work with Unicode as much as possible
impl fmt::Display for Rotor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = self.alphabet.iter().collect();
        s.push_str(&format!(" ({})",self.pins.iter().collect::<String>()));
        write!(f, "{}", s)
    }
}


//The rotor alphabets all have coprime lengths
lazy_static! {
    pub static ref M209_ROTORS: [Rotor; 6] = {
        [Rotor::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ",15),
         Rotor::new("ABCDEFGHIJKLMNOPQRSTUVXYZ",14),
         Rotor::new("ABCDEFGHIJKLMNOPQRSTUVX",13),
         Rotor::new("ABCDEFGHIJKLMNOPQRSTU",12),
         Rotor::new("ABCDEFGHIJKLMNOPQRS",11),
         Rotor::new("ABCDEFGHIJKLMNOPQ",10),
        ]
    };
}