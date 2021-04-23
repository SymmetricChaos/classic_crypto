
use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::ciphers::enigma::enigma::Rotor;

lazy_static! {
    pub static ref ENIGMA_ROTORS: HashMap<String,Rotor> = {
        let mut m = HashMap::new();
        m.insert("I".to_string(), Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 16));
        m.insert("II".to_string(), Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", 4));
        m.insert("III".to_string(), Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", 21));
        m.insert("IV".to_string(), Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", 9));
        m.insert("V".to_string(), Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", 25));
        m
    };

    pub static ref ENIGMA_REFLECTORS: HashMap<String,Rotor> = {
        let mut m = HashMap::new();
        m.insert("A".to_string(), Rotor::new("EJMZALYXVBWFCRQUONTSPIKHGD", 26));
        m.insert("B".to_string(), Rotor::new("YRUHQSLDPXNGOKMIEBFZCWVJAT", 26));
        m.insert("C".to_string(), Rotor::new("FVPJIAOYEDRZXWGCTKUQSBNMHL", 26));
        m
    };
}