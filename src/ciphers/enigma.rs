use std::fmt;
use std::collections::{VecDeque,HashMap};
use lazy_static::lazy_static;
//use rand::Rng;



#[derive(Clone,Debug)]
pub struct Rotor {
    wiring: VecDeque<char>,
    notch: char,
}

impl Rotor {
    pub fn new(alphabet: &str, notch: char) -> Rotor {
        let wiring: VecDeque<char> = alphabet.chars().into_iter().collect();
        Rotor{ wiring, notch }
    }

    pub fn step(&mut self) {
        self.wiring.rotate_left(1);
    }

    pub fn swap(&self, character: char) -> char {
        let n = character as usize-65;
        self.wiring[n]
    }

    pub fn swap_inv(&self, character: char) -> char {
        // Find the position, add 65 to align it with the ASCII chart, then cast to char
        (self.wiring.iter().position(|&x| x == character).unwrap() + 65) as u8 as char
    }

}

lazy_static! {
    pub static ref ROTOR_I: Rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Q');
    pub static ref ROTOR_II: Rotor = Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", 'E');
    pub static ref ROTOR_III: Rotor = Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", 'V');
    pub static ref ROTOR_IV: Rotor = Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", 'J');
    pub static ref ROTOR_V: Rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Z'); 
    pub static ref REFLECTOR_A: Rotor = Rotor::new("EJMZALYXVBWFCRQUONTSPIKHGD", '#');
    pub static ref REFLECTOR_B: Rotor = Rotor::new("YRUHQSLDPXNGOKMIEBFZCWVJAT", '#');
    pub static ref REFLECTOR_C: Rotor = Rotor::new("FVPJIAOYEDRZXWGCTKUQSBNMHL", '#');
}





#[derive(Clone,Debug)]
pub struct Plugboard {
    wiring: HashMap<char,char>
}

impl Plugboard {
    pub fn new(pairs: Vec<(char,char)>) -> Plugboard {
        let mut wiring = HashMap::new();
        for (p1, p2) in pairs {
            if wiring.contains_key(&p1) || wiring.contains_key(&p2) {
                panic!("Plugboard settings cannot overlap.")
            }
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

    /* pub fn random(&self) -> Plugboard {

    } */
}





#[derive(Clone,Debug)]
pub struct Settings {
    plugboard: Plugboard,
    rotors: (Rotor,Rotor,Rotor),
    ring_positions: Vec<u8>,
}

impl Settings {
    pub fn new(plugboard: Plugboard, rotors: (Rotor,Rotor,Rotor), ring_positions: Vec<u8>) -> Settings {
        Settings{ plugboard, rotors, ring_positions }
    }

    // Need to include double-stepping
    pub fn advance_rotors(&mut self) {
        self.rotors.0.step();
        if self.rotors.0.wiring.iter().last().unwrap() == &self.rotors.0.notch {
            self.rotors.1.step();
            if self.rotors.1.wiring.iter().last().unwrap() == &self.rotors.1.notch {
                self.rotors.2.step();
            }
        }
    }
}





pub struct Enigma {
    key: Settings,
}

impl Enigma {
    pub fn new(key: Settings) -> Enigma {
        Enigma{ key }
    }



/*     pub fn encode(&mut self, rotor_positions: Vec<u8>, text: &str) -> Result<String,Error> {

    } 
*/
}

/* impl fmt::Display for Enigma {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Enigma Cipher\nPlugboard: {}\nRotors: {}\nRing Positions:{}",self.key.plugboard, self.key.rotors, self.key.ring_positions)
    }
} 
*/





#[test]
fn plugboard() {
    let board = Plugboard::new(vec![('A','B'),('X','Z')]);

    println!("{} -> {}",'A',board.swap('A'));
    println!("{} -> {}",'B',board.swap('B'));
    println!("{} -> {}",'C',board.swap('C'));
}


#[test]
fn single_rotor() {
    let mut rotor = ROTOR_I.clone();

    println!("{} -> {}, then step", 'A', rotor.swap('A'));
    rotor.step();
    println!("{} -> {}, then step", 'A', rotor.swap('A'));
    rotor.step();
    println!("{} -> {}, then step", 'A', rotor.swap('A'));
    rotor.step();
}

#[test]
fn single_rotor_inv() {
    let mut rotor = ROTOR_I.clone();

    println!("{} -> {}, then step", 'A', rotor.swap_inv('A'));
    rotor.step();
    println!("{} -> {}, then step", 'A', rotor.swap_inv('A'));
    rotor.step();
    println!("{} -> {}, then step", 'A', rotor.swap_inv('A'));
    rotor.step();
}


#[test]
fn triple_rotor() {
    let rotor1 = ROTOR_I.clone();
    let rotor2 = ROTOR_II.clone();
    let rotor3 = ROTOR_III.clone();

    let rotors = vec![rotor1, rotor2, rotor3];
    let mut c = 'A';
    for r in rotors {
        c = r.swap(c);
    }

    assert_eq!(c,'G');

    println!("{} -> {}", 'A', c);

}

#[test]
fn full_rotor() {
    let rotor1 = ROTOR_I.clone();
    let rotor2 = ROTOR_II.clone();
    let rotor3 = ROTOR_III.clone();
    let reflector = Rotor::new("EJMZALYXVBWFCRQUONTSPIKHGD", '#');

    let rotors = vec![&rotor1, &rotor2, &rotor3];
    let inv_rotors = vec![&rotor3, &rotor2, &rotor1];

    let mut c = 'A';
    for r in rotors {
        c = r.swap(c);
    }
    c = reflector.swap(c);
    for r in inv_rotors {
        c = r.swap(c);
    }

    assert_eq!(c,'X');

    println!("{} -> {}", 'A', c);

}