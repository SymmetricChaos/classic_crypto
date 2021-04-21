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
    wiring: HashMap<char,char>,
    pairs: Vec<(char,char)>
}

impl Plugboard {
    pub fn new(pairs: Vec<(char,char)>) -> Plugboard {
        let mut wiring = HashMap::new();
        for (p1, p2) in pairs.clone() {
            if wiring.contains_key(&p1) || wiring.contains_key(&p2) {
                panic!("Plugboard settings cannot overlap.")
            }
            wiring.insert(p1,p2);
            wiring.insert(p2,p1);
        }
        Plugboard{ wiring, pairs }
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
        write!(f, "Plugboard: {:?}",self.pairs)
    }
} 





#[derive(Clone,Debug)]
pub struct Settings {
    plugboard: Plugboard,
    rotors: (Rotor,Rotor,Rotor),
    reflector: Rotor,
    ring_positions: (u8,u8,u8),
}

impl Settings {
    pub fn new(plugboard: Plugboard, rotors: (Rotor,Rotor,Rotor), reflector: Rotor, ring_positions: (u8,u8,u8)) -> Settings {
        Settings{ plugboard, rotors, reflector, ring_positions }
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

    fn encode_char(&self, c: char) -> char {
        let mut x = c;
        //println!("start {}",x);
        x = self.plugboard.swap(x);
        //println!("P {}",x);
        x = self.rotors.0.swap(x);
        //println!("R1 {}",x);
        x = self.rotors.1.swap(x);
        //println!("R2 {}",x);
        x = self.rotors.2.swap(x);
        //println!("R3 {}",x);
        x = self.reflector.swap(x);
        //println!("REF {}",x);
        x = self.rotors.2.swap_inv(x);
        //println!("R3 {}",x);
        x = self.rotors.1.swap_inv(x);
        //println!("R2 {}",x);
        x = self.rotors.0.swap_inv(x);
        //println!("R1 {}",x);
        x =self.plugboard.swap(x);
        //println!("P {}",x);
        x
    }
}





pub struct Enigma {
    settings: Settings,
}

impl Enigma {
    pub fn new(settings: Settings) -> Enigma {
        Enigma{ settings }
    }


    pub fn encode(&mut self, rotor_positions: Vec<u8>, text: &str) -> String {
        let mut out = Vec::new();
        for c in text.chars() {
            out.push(self.settings.encode_char(c));
            self.settings.advance_rotors();
        }
        let message: String = out.iter().collect();
        message
    } 

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
fn settings() {
    let plugboard = Plugboard::new(vec![('A','B'),('X','Z')]);
    let rotor1 = ROTOR_I.clone();
    let rotor2 = ROTOR_II.clone();
    let rotor3 = ROTOR_III.clone();
    let reflector = REFLECTOR_A.clone();

    let rotors = (rotor1, rotor2, rotor3);
    let ring_positions = (0,0,0);

    let mut s = Settings::new(plugboard, rotors, reflector, ring_positions );

    let text = "AAA".chars();
    for c in text {
        print!("{}",s.encode_char(c));
        s.advance_rotors();
    }
    println!("");

    println!("we should get:\nEKV")


}

