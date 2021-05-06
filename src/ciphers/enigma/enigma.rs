use std::fmt;
use std::{ fs::File, io::{Write,Error, Read}};
use std::collections::HashMap;
//use rand::Rng;

// References
// This is the M3 Enigma
// https://github.com/aurbano/EnigmaM3_py
// https://cryptii.com/pipes/EnigmaM3-machine

fn char_to_usize(c: char) -> usize {
    (c as u8 as usize) - 65
}

fn usize_to_char(n: usize) -> char {
    (n + 65) as u8 as char
}



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

    // Signal starts on the right amd goes through the rotor then back
    // We will use and return usize instead of char to avoid constantly converting types
    // There MUST be an easier way to do this.
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

impl fmt::Display for Rotor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",self.wiring_display)
    }
} 





#[derive(Clone,Debug)]
pub struct Plugboard {
    wiring: HashMap<char,char>,
    pairs: String
}

fn parse_plugboard(pairs: &str) -> HashMap<char,char> {
    let mut wiring = HashMap::new();
    let digraphs = pairs.split(" ");
    for d in digraphs {
        if d.len() != 2 {
            panic!("plugboard settings must be pairs of letters")
        }
        let mut cs = d.chars();
        let a = cs.next().unwrap();
        let b = cs.next().unwrap();
        wiring.insert(a,b);
        wiring.insert(b,a);
    }
    wiring
}

impl Plugboard {
    pub fn new(pairs: &str) -> Plugboard {
        let wiring = match pairs.len() == 0 {
            true =>  HashMap::<char,char>::new(),
            false => parse_plugboard(pairs),
        };
        Plugboard{ wiring, pairs: pairs.to_string() }
    }

    pub fn swap(&self, character: char) -> char {
        if self.wiring.contains_key(&character) {
            self.wiring[&character]
        } else {
            character
        }
    }

    /* pub fn random(&self, n: u32) -> Plugboard {

    } */
}

impl fmt::Display for Plugboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Plugboard: {}",self.pairs)
    }
} 





#[derive(Clone,Debug)]
pub struct EnigmaM3<'a> {
    plugboard: Plugboard,
    rotors: (Rotor<'a>,Rotor<'a>,Rotor<'a>),
    reflector: Rotor<'a>,
    ring_positions: (usize,usize,usize),
}

impl<'a> EnigmaM3<'a> {
    // Note that rotor positions are not provided here. Only the key settings are.
    pub fn new(plugs: &str, rotors: (Rotor<'a>, Rotor<'a>, Rotor<'a>), reflector: Rotor<'a>, ring_positions: (usize,usize,usize)) -> EnigmaM3<'a> {
        let plugboard = Plugboard::new(plugs);
        EnigmaM3{ plugboard, rotors, reflector, ring_positions }
    }

    pub fn set_rotors(&mut self, rotor_positions: (usize,usize,usize)) {
        self.rotors.0.set_position(rotor_positions.0);
        self.rotors.1.set_position(rotor_positions.1);
        self.rotors.2.set_position(rotor_positions.2);
    }

    pub fn print_rotor_positions(&self) {
        println!("{} {} {}",
            self.rotors.0.position,
            self.rotors.1.position,
            self.rotors.2.position)
    }

    pub fn print_ring_positions(&self) {
        println!("{} {} {}",
            self.ring_positions.0,
            self.ring_positions.1,
            self.ring_positions.2)
    }

    // Need to validate double-stepping
    fn advance_rotors(&mut self) {
        let mut on_notch = self.rotors.2.position == self.rotors.2.notch.0 || self.rotors.2.position == self.rotors.2.notch.1;
        self.rotors.2.step();
        if on_notch {
            on_notch = self.rotors.1.position == self.rotors.1.notch.0 || self.rotors.1.position == self.rotors.1.notch.1;
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
        x = self.reflector.encrypt_rtl(x);
        x = self.rotors.0.encrypt_ltr(x);
        x = self.rotors.1.encrypt_ltr(x);
        x = self.rotors.2.encrypt_ltr(x);
        self.plugboard.swap(usize_to_char(x))
    }

    // Rotor positions are meant to be different for each message so .set_rotors() should be called before use
    pub fn encrypt(&mut self, text: &str) -> String {

        self.rotors.0.set_ring(self.ring_positions.0);
        self.rotors.1.set_ring(self.ring_positions.1);
        self.rotors.2.set_ring(self.ring_positions.2);
        
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

    pub fn encrypt_file(&mut self, source: &str, target: &str) -> Result<(),Error> {

        let mut target_file = File::create(target.to_string())?;
    
        let mut source_file = File::open(source)?;
        let mut source_text = String::new();
        source_file.read_to_string(&mut source_text)?;
    
        let ciphertext = self.encrypt(&source_text);
    
        target_file.write(ciphertext.as_bytes())?;

        Ok(())
    }
}

impl fmt::Display for EnigmaM3<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Enigma M3\n{}\nRotor 1: {} ({})\nRotor 2: {} ({})\nRotor 2: {} ({})",
            self.plugboard,
            self.rotors.0,
            self.ring_positions.0,
            self.rotors.1,
            self.ring_positions.1,
            self.rotors.2,
            self.ring_positions.2)
    }
}