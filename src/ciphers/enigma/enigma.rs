use std::fmt;
use std::{ fs::File, io::{Write,Error, Read}};
use std::collections::HashMap;
use lazy_static::lazy_static;
//use rand::Rng;

// References
// This is the M3 Enigma
// https://github.com/aurbano/enigma_py
// https://cryptii.com/pipes/enigma-machine

fn char_to_usize(c: char) -> usize {
    (c as u8 as usize) - 65
}

fn usize_to_char(n: usize) -> char {
    (n + 65) as u8 as char
}



#[derive(Clone,Debug)]
pub struct Rotor {
    wiring_rtl: [usize; 26],
    wiring_ltr: [usize; 26],
    notch: usize,
    position: usize,
    ring: usize,
    wiring_display: String,
}

impl Rotor {
    pub fn new(wiring: &str, notch: usize) -> Rotor {
        let mut wiring_rtl: [usize; 26] = [0; 26];
        let mut wiring_ltr: [usize; 26] = [0; 26];
        for w in wiring.chars().map(|x| ((x as u8) - 65) as usize ).enumerate() {
            wiring_rtl[w.0] = w.1;
            wiring_ltr[w.1] = w.0;
        }
        Rotor{ wiring_rtl, wiring_ltr, notch, position: 0, ring: 0, wiring_display: wiring.to_string() }
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

lazy_static! {
    pub static ref ROTOR_I: Rotor =   Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 16);
    pub static ref ROTOR_II: Rotor =  Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", 4);
    pub static ref ROTOR_III: Rotor = Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", 21);
    pub static ref ROTOR_IV: Rotor =  Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", 9);
    pub static ref ROTOR_V: Rotor =   Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", 25); 
    pub static ref REFLECTOR_A: Rotor = Rotor::new("EJMZALYXVBWFCRQUONTSPIKHGD", 26);
    pub static ref REFLECTOR_B: Rotor = Rotor::new("YRUHQSLDPXNGOKMIEBFZCWVJAT", 26);
    pub static ref REFLECTOR_C: Rotor = Rotor::new("FVPJIAOYEDRZXWGCTKUQSBNMHL", 26);
}


impl fmt::Display for Rotor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",self.wiring_display)
    }
} 




// TODO: Introduce function to parse an &str to create the pairs
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

    /* pub fn random(&self) -> Plugboard {

    } */
}

impl fmt::Display for Plugboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Plugboard: {}",self.pairs)
    }
} 





#[derive(Clone,Debug)]
pub struct Enigma {
    plugboard: Plugboard,
    rotors: (Rotor,Rotor,Rotor),
    reflector: Rotor,
    ring_positions: (usize,usize,usize),
}

impl Enigma {
    // Note that rotor positions are not provided here. Only the key settings are.
    pub fn new(plugs: &str, rotors: (Rotor,Rotor,Rotor), reflector: Rotor, ring_positions: (usize,usize,usize)) -> Enigma {
        let plugboard = Plugboard::new(plugs);
        Enigma{ plugboard, rotors, reflector, ring_positions }
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
        let mut on_notch = self.rotors.2.position == self.rotors.2.notch;
        self.rotors.2.step();
        if on_notch {
            on_notch = self.rotors.1.position == self.rotors.1.notch;
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

    // Rotor positions are meant to be different for each message so here they are supplied when .encrypt() is called
    // There is no .decrypt() method as the Enigma was involutive and thus needed no decrypt setting
    pub fn encrypt(&mut self, text: &str, rotor_positions: (usize,usize,usize)) -> String {

        self.rotors.0.set_position(rotor_positions.0);
        self.rotors.1.set_position(rotor_positions.1);
        self.rotors.2.set_position(rotor_positions.2);

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

    pub fn encrypt_file(&mut self, source: &str, target: &str, rotor_positions: (usize,usize,usize)) -> Result<(),Error> {

        let mut target_file = File::create(target.to_string())?;
    
        let mut source_file = File::open(source)?;
        let mut source_text = String::new();
        source_file.read_to_string(&mut source_text)?;
    
        let ciphertext = self.encrypt(&source_text, rotor_positions);
    
        target_file.write(ciphertext.as_bytes())?;

        Ok(())
    }
}

impl fmt::Display for Enigma {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Enigma Machine\n{}\nRotor 1: {} ({})\nRotor 2: {} ({})\nRotor 2: {} ({})",
            self.plugboard,
            self.rotors.0,
            self.ring_positions.0,
            self.rotors.1,
            self.ring_positions.1,
            self.rotors.2,
            self.ring_positions.2)
    }
} 




#[test]
fn plugboard() {
    let board = Plugboard::new("AB XY");

    println!("{} -> {}",'A',board.swap('A'));
    println!("{} -> {}",'B',board.swap('B'));
    println!("{} -> {}",'C',board.swap('C'));
}

#[test]
fn single_rotor() {
    let rotor = ROTOR_III.clone();
    println!("{}",rotor);
    let c = char_to_usize('A');
    println!("{} -> {}", 'A', usize_to_char(rotor.encrypt_rtl(c)));
    println!("should get: A -> B")
}

#[test]
fn single_rotor_stepping() {
    let mut rotor = ROTOR_III.clone();
    println!("{}",rotor);
    rotor.set_ring(1);
    rotor.set_position(0);

    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encrypt_rtl(0)));
    
    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encrypt_rtl(0)));
    
    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encrypt_rtl(0)));

    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encrypt_rtl(0)));

    println!("right column: BCDE\nleft column: BCDE")
}

#[test]
fn single_rotor_stepping_2() {
    let mut rotor = ROTOR_II.clone();
    println!("{}",rotor);
    rotor.set_ring(25);
    rotor.set_position(25);

    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encrypt_rtl(0)));
    
    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encrypt_rtl(0)));
    
    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encrypt_rtl(0)));

    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encrypt_rtl(0)));

    println!("right column: ABCD\nleft column: IBHO")
}

#[test]
fn enigma() {
    let rotor1 = ROTOR_IV.clone();
    let rotor2 = ROTOR_II.clone();
    let rotor3 = ROTOR_V.clone();
    let rotors = (rotor1, rotor2, rotor3);
    let reflector = REFLECTOR_B.clone();
    let ring_positions = (14,22,25);

    let mut s = Enigma::new( "EJ OY IV AQ KW FX MT PS LU BD", rotors, reflector, ring_positions );

    println!("\n{}\n",s);

    let text = "AAAAAAAAAAAAAAAAAAAAAAAAAA";
    let out = s.encrypt(text,(0,0,0));
    println!("{}\n{}",text,out);
    assert_eq!(&out,"VDDXSYJOVCQYJSDJMLONNSSJQI");

    // Confirm involution property
    let text = "VDDXSYJOVCQYJSDJMLONNSSJQI";
    let out = s.encrypt(text,(0,0,0));
    assert_eq!(&out,"AAAAAAAAAAAAAAAAAAAAAAAAAA");
}