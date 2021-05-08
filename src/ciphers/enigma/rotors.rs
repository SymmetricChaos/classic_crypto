use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};

#[derive(Clone,Debug,Copy)]
pub struct Rotor<'a> {
    wiring_rtl: [usize; 26],
    wiring_ltr: [usize; 26],
    notch: (usize,usize),
    position: usize,
    ring: usize,
    wiring_display: &'a str,
}

impl Rotor<'_> {
    pub fn new(wiring: &str, notch: (usize,usize)) -> Rotor {
        let mut wiring_rtl: [usize; 26] = [0; 26];
        let mut wiring_ltr: [usize; 26] = [0; 26];
        for w in wiring.chars().map(|x| ((x as u8) - 65) as usize ).enumerate() {
            wiring_rtl[w.0] = w.1;
            wiring_ltr[w.1] = w.0;
        }
        Rotor{ wiring_rtl, wiring_ltr, notch, position: 0, ring: 0, wiring_display: wiring }
    }

    pub fn step(&mut self) {
        self.position = (self.position + 1) % 26
    }

    pub fn set_ring(&mut self, n: usize) {
        self.ring = n;
    }

    pub fn set_position(&mut self, n: usize) {
        self.position = n;
    }

    pub fn get_ring(&self) -> usize {
        self.ring
    }

    pub fn get_position(&self) -> usize {
        self.position
    }

    pub fn get_notch(&self) -> (usize,usize) {
        self.notch
    }

    pub fn wiring(&self) -> String {
        self.wiring_display.to_string()
    }

    // Signal starts on the right amd goes through the rotor then back
    // We will use and return usize instead of char to avoid constantly converting types
    pub fn encrypt_rtl(&self, entry: usize) -> usize {
        let inner_position = (26+entry+self.position-self.ring)%26;
        let inner = self.wiring_rtl[inner_position];
        (inner+26-self.position+self.ring) % 26
    }

    pub fn encrypt_ltr(&self, entry: usize) -> usize {
        let inner_position = (26+entry+self.position-self.ring)%26;
        let inner = self.wiring_ltr[inner_position];
        (inner+26-self.position+self.ring) % 26
    }

}

// This could be simplified since all the real rotors used ASCII characters but this library tries to work with Unicode as much as possible
impl fmt::Display for Rotor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for (pos,letter) in self.wiring_display.chars().enumerate() {
            if pos == self.position {
                s.push_str(&format!("[{}]",letter));
            } else {
                s.push(letter)
            }
        }
        s.push_str(&format!(" ({})",self.ring));
        write!(f, "{}",s)
    }
}


#[derive(Clone,Debug,Copy)]
pub struct Reflector<'a> {
    wiring: [usize; 26],
    wiring_display: &'a str,
}

impl Reflector<'_> {
    pub fn new(wiring: &str) -> Reflector {
        let mut wiring_internal: [usize; 26] = [0; 26];
        for w in wiring.chars().map(|x| ((x as u8) - 65) as usize ).enumerate() {
            wiring_internal[w.0] = w.1;
        }
        Reflector{ wiring: wiring_internal, wiring_display: wiring }
    }

    // We take and return usize to be consistent with Rotor
    pub fn encrypt(&self, entry: usize) -> usize {
        self.wiring[entry]
    }

    pub fn wiring(&self) -> String {
        self.wiring_display.to_string()
    }

}

impl fmt::Display for Reflector<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",self.wiring_display)
    }
} 



lazy_static! {
    pub static ref ROTORS: HashMap<&'static str, Rotor<'static>> = {
        let mut m = HashMap::new();
        m.insert("I",    Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", (16,16) ));
        m.insert("II",   Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", (4,4) ));
        m.insert("III",  Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", (21,21) ));
        m.insert("IV",   Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", (9,9) ));
        m.insert("V",    Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", (25,25) ));
        m.insert("VI",   Rotor::new("JPGVOUMFYQBENHZRDKASXLICTW", (12,25) ));
        m.insert("VII",  Rotor::new("NZJHGRCXMYSWBOUFAIVLPEKQDT", (12,25) ));
        m.insert("VIII", Rotor::new("FKQHTLXOCBJSPDZRAMEWNIUYGV", (12,25) ));
        m
    };

    pub static ref REFLECTORS: HashMap<&'static str, Reflector<'static>> = {
        let mut m = HashMap::new();
        m.insert("Alpha",  Reflector::new("LEYJVCNIXWPBQMDRTAKZGFUHOS"));
        m.insert("Beta",   Reflector::new("FSOKANUERHMBTIYCWLQPZXVGJD"));
        m.insert("A",      Reflector::new("EJMZALYXVBWFCRQUONTSPIKHGD"));
        m.insert("B",      Reflector::new("YRUHQSLDPXNGOKMIEBFZCWVJAT"));
        m.insert("C",      Reflector::new("FVPJIAOYEDRZXWGCTKUQSBNMHL"));
        m.insert("B-thin", Reflector::new("ENKQAUYWJICOPBLMDXZVFTHRGS"));
        m.insert("C-thin", Reflector::new("RDOBJNTKVEHMLFCWZAXGYIPSUQ"));
        m
    };
}