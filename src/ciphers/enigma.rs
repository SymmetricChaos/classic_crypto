use std::fmt;
use std::collections::{VecDeque,HashMap};





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

    pub fn swap(&self, character: char) -> char {
        let n = character as usize-65;
        self.wiring[n]
    }

    pub fn swap_inv(&self, character: char) -> char {
        // Find the position, add 65 to align it with the ASCII chart, then cast to char
        (self.wiring.iter().position(|&x| x == character).unwrap() + 65) as u8 as char
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
    let mut rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ");

    println!("{} -> {}, then step", 'A', rotor.swap('A'));
    rotor.step();
    println!("{} -> {}, then step", 'A', rotor.swap('A'));
    rotor.step();
    println!("{} -> {}, then step", 'A', rotor.swap('A'));
    rotor.step();
}

#[test]
fn single_rotor_inv() {
    let mut rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ");

    println!("{} -> {}, then step", 'A', rotor.swap_inv('A'));
    rotor.step();
    println!("{} -> {}, then step", 'A', rotor.swap_inv('A'));
    rotor.step();
    println!("{} -> {}, then step", 'A', rotor.swap_inv('A'));
    rotor.step();
}


#[test]
fn triple_rotor() {
    let rotor1 = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ");
    let rotor2 = Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE");
    let rotor3 = Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO");

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
    let rotor1 = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ");
    let rotor2 = Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE");
    let rotor3 = Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO");
    let reflector = Rotor::new("EJMZALYXVBWFCRQUONTSPIKHGD");

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