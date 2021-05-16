use std::fmt;
use std::{ fs::File, io::{Write,Error, Read}};
use std::collections::HashMap;

use super::{Rotor, rotors::Reflector};


// The M209 was a reciprocal cipher

fn char_to_usize(c: char) -> usize {
    (c as u8 as usize) - 65
}

fn usize_to_char(n: usize) -> char {
    (n + 65) as u8 as char
}

/*
The rotor alphabets all have coprime lengths
"ABCDEFGHIJKLMNOPQRSTUVWXYZ"
"ABCDEFGHIJKLMNOPQRSTUVXYZ"
"ABCDEFGHIJKLMNOPQRSTUVX"
"ABCDEFGHIJKLMNOPQRSTU"
"ABCDEFGHIJKLMNOPQRS"
"ABCDEFGHIJKLMNOPQ"
*/

/*
Pin Settings
*/

/*
Lug Settings
*/



#[derive(Clone,Debug)]
pub struct M209<'a> {
    plugboard: Plugboard<'a>,
    rotors: (Rotor<'a>,Rotor<'a>,Rotor<'a>),
    reflector: Reflector<'a>,
    ring_positions: (usize,usize,usize),
}

impl<'a> EnigmaM3<'a> {
    // Note that rotor positions are not provided here. Only the key settings are.
    pub fn new(plugs: &'a str, rotors: (Rotor<'a>, Rotor<'a>, Rotor<'a>), reflector: Reflector<'a>, ring_positions: (usize,usize,usize)) -> EnigmaM3<'a> {
        let plugboard = Plugboard::new(plugs);
        let mut a = rotors.0.clone();
        a.set_ring(ring_positions.0);
        let mut b = rotors.1.clone();
        b.set_ring(ring_positions.1);
        let mut c = rotors.2.clone();
        c.set_ring(ring_positions.2);
        EnigmaM3{ plugboard, rotors: (a,b,c), reflector, ring_positions }
    }

    pub fn set_rotors(&mut self, rotor_positions: (usize,usize,usize)) {
        self.rotors.0.set_position(rotor_positions.0);
        self.rotors.1.set_position(rotor_positions.1);
        self.rotors.2.set_position(rotor_positions.2);
    }

    pub fn show_rotors(&self) -> String {
        format!("Rotor 1: {}\nRotor 2: {}\nRotor 3: {}",
            self.rotors.0,
            self.rotors.1,
            self.rotors.2,)
    }

    pub fn show_reflector(&self) -> String {
        format!("Reflector: {}",
            self.reflector)
    }

    // Need to validate double-stepping
    fn advance_rotors(&mut self) {
        let mut on_notch = self.rotors.2.get_position() == self.rotors.2.get_notch().0 || self.rotors.2.get_position() == self.rotors.2.get_notch().1;
        self.rotors.2.step();
        if on_notch {
            on_notch = self.rotors.1.get_position() == self.rotors.1.get_notch().0 || self.rotors.1.get_position() == self.rotors.1.get_notch().1;
            self.rotors.1.step();
            if on_notch {
                self.rotors.0.step();
            }
        }
    }

    // Notice that the signal goes through the rotors starting on the right with the 3rd rotor, 
    // then through the reflector, and back through from left to right starting with the 1st rotor
    fn encrypt_char(&mut self, c: char) -> char {
        self.advance_rotors();
        //self.get_rotor_positions();
        let mut x = char_to_usize(self.plugboard.swap(c));
        x = self.rotors.2.encrypt_rtl(x);
        x = self.rotors.1.encrypt_rtl(x);
        x = self.rotors.0.encrypt_rtl(x);
        x = self.reflector.encrypt(x);
        x = self.rotors.0.encrypt_ltr(x);
        x = self.rotors.1.encrypt_ltr(x);
        x = self.rotors.2.encrypt_ltr(x);
        self.plugboard.swap(usize_to_char(x))
    }

    pub fn encrypt_file(&mut self, source: &str, target: &str) -> Result<(),Error> {

        let mut target_file = File::create(target.to_string())?;
    
        let mut source_file = File::open(source)?;
        let mut source_text = String::new();
        source_file.read_to_string(&mut source_text)?;
    
        let ciphertext = self.encrypt(&source_text);
    
        target_file.write(ciphertext.as_bytes())?;

        Ok(())
    }

    // Rotor positions are meant to be different for each message so .set_rotors() should be called before use
    pub fn encrypt(&mut self, text: &str) -> String {      
        let letters = text.chars();
        let mut out = String::new();
        for c in letters {
            out.push(self.encrypt_char(c));
        }
        out
    }

    // This method is just for compatibility as the Enigma machines were all involutive, encryption and decryption were the same process
    pub fn decrypt(&mut self, text: &str) -> String {
        self.encrypt(text)
    }

}

// For this to work we 
//impl crate::auxiliary::Cipher for EnigmaM3<'_> {}

impl fmt::Display for EnigmaM3<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Enigma M3\n{}\n{}\n{}",
            self.plugboard,
            self.show_rotors(),
            self.show_reflector())
    }
}