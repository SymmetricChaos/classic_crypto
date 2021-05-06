use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::ciphers::enigma::enigma::Rotor;

lazy_static! {
    pub static ref ROTOR: HashMap<&'static str, Rotor<'static>> = {
        let mut m = HashMap::new();
        m.insert("I", Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", (16,16) ));
        m.insert("II", Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", (4,4) ));
        m.insert("III", Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", (21,21) ));
        m.insert("IV", Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", (9,9) ));
        m.insert("V", Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", (25,25) ));
        m.insert("VI", Rotor::new("JPGVOUMFYQBENHZRDKASXLICTW", (12,25) ));
        m.insert("VII", Rotor::new("NZJHGRCXMYSWBOUFAIVLPEKQDT", (12,25) ));
        m.insert("VIII", Rotor::new("FKQHTLXOCBJSPDZRAMEWNIUYGV", (12,25) ));
        m
    };

    pub static ref REFLECTOR: HashMap<&'static str, Rotor<'static>> = {
        let mut m = HashMap::new();
        m.insert("Alpha", Rotor::new("LEYJVCNIXWPBQMDRTAKZGFUHOS", (26,26)));
        m.insert("Beta", Rotor::new("FSOKANUERHMBTIYCWLQPZXVGJD", (26,26)));
        m.insert("A", Rotor::new("EJMZALYXVBWFCRQUONTSPIKHGD", (26,26)));
        m.insert("B", Rotor::new("YRUHQSLDPXNGOKMIEBFZCWVJAT", (26,26)));
        m.insert("C", Rotor::new("FVPJIAOYEDRZXWGCTKUQSBNMHL", (26,26)));
        m.insert("B-thin", Rotor::new("ENKQAUYWJICOPBLMDXZVFTHRGS", (26,26)));
        m.insert("C-thin", Rotor::new("RDOBJNTKVEHMLFCWZAXGYIPSUQ", (26,26)));
        m
    };
}