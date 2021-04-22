use std::fmt;
use std::collections::HashMap;
use lazy_static::lazy_static;
//use rand::Rng;

// Reference
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
    alpha: [usize; 26],
    notch: usize,
    position: usize,
    ring: usize,
    wiring_display: String,
}

impl Rotor {
    pub fn new(wiring: &str, notch: usize) -> Rotor {
        let mut wiring_rtl: [usize; 26] = [0; 26];
        let mut wiring_ltr: [usize; 26] = [0; 26];
        let mut alpha: [usize; 26] = [0; 26];
        for w in wiring.chars().map(|x| ((x as u8) - 65) as usize ).enumerate() {
            wiring_rtl[w.0] = w.1;
            wiring_ltr[w.1] = w.0;
            alpha[w.0] = w.0;
        }
        Rotor{ wiring_rtl, wiring_ltr, alpha, notch, position: 0, ring: 0, wiring_display: wiring.to_string() }
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

    pub fn get_ring(&mut self) -> usize {
        self.ring
    }

    pub fn get_position(&mut self) -> usize {
        self.position
    }

    // Signal starts on the right amd goes through the rotor then back
    // We will use and return usize instead of char to avoid constantly converting types
    // There MUST be an easier way to do this.
    // The RTL portion works. There may be issues with the LTR portion.
    pub fn encode_rtl(&self, entry: usize) -> usize {
        let inner_position = (26+entry+self.position-self.ring)%26;
        let inner = self.wiring_rtl[inner_position];
        let outer = (self.alpha.iter().position(|x| x == &inner).unwrap()+26-self.position+self.ring) % 26;
        outer
    }

    pub fn encode_ltr(&self, entry: usize) -> usize {
        let inner_position = (26+entry+self.position-self.ring)%26;
        let inner = self.alpha[inner_position];
        let outer = (self.wiring_rtl.iter().position(|x| x == &inner).unwrap()+26-self.position+self.ring) % 26;
        outer
    }

}

lazy_static! {
    pub static ref ROTOR_I: Rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 16);
    pub static ref ROTOR_II: Rotor = Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", 4);
    pub static ref ROTOR_III: Rotor = Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", 21);
    pub static ref ROTOR_IV: Rotor = Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", 9);
    pub static ref ROTOR_V: Rotor = Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", 25); 
    pub static ref REFLECTOR_A: Rotor = Rotor::new("EJMZALYXVBWFCRQUONTSPIKHGD", 26);
    pub static ref REFLECTOR_B: Rotor = Rotor::new("YRUHQSLDPXNGOKMIEBFZCWVJAT", 26);
    pub static ref REFLECTOR_C: Rotor = Rotor::new("FVPJIAOYEDRZXWGCTKUQSBNMHL", 26);
}


impl fmt::Display for Rotor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rotor: {:?}\n{:?}\n{:?}",self.wiring_display,self.wiring_rtl,self.alpha)
    }
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
        if self.rotors.0.position == self.rotors.0.notch {
            self.rotors.1.step();
            if self.rotors.1.position == self.rotors.1.notch {
                self.rotors.2.step();
            }
        }
    }

     fn encode_char(&self, c: char) -> char {
        let mut x = char_to_usize(self.plugboard.swap(c));
        x = self.rotors.0.encode_rtl(x);
        x = self.rotors.1.encode_rtl(x);
        x = self.rotors.2.encode_rtl(x);
        x = self.reflector.encode_rtl(x);
        x = self.rotors.2.encode_ltr(x);
        x = self.rotors.1.encode_ltr(x);
        x = self.rotors.0.encode_ltr(x);
        self.plugboard.swap(usize_to_char(x))
        
    }
}





pub struct Enigma {
    settings: Settings,
}

impl Enigma {
    pub fn new(settings: Settings) -> Enigma {
        Enigma{ settings }
    }


/*     pub fn encode(&mut self, rotor_positions: Vec<u8>, text: &str) -> String {
        let mut out = Vec::new();
        for c in text.chars() {
            out.push(self.settings.encode_char(c));
            self.settings.advance_rotors();
        }
        let message: String = out.iter().collect();
        message
    }  */

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
    let rotor = ROTOR_III.clone();
    println!("{}",rotor);
    let c = char_to_usize('A');
    println!("{} -> {}", 'A', usize_to_char(rotor.encode_rtl(c)));
    println!("shuold get: A -> B")
}

#[test]
fn single_rotor_rlt_and_ltr() {
    let rotor = ROTOR_I.clone();
    println!("{}",rotor);

    println!("LTR");
    for entry in 0..26usize {
        let encoding = rotor.encode_ltr(entry);
        println!("{} -> {}", usize_to_char(entry), usize_to_char(encoding));
    }

    println!("\n\nRTL");
    for entry in 0..26usize {
        let encoding = rotor.encode_rtl(entry);
        println!("{} -> {}", usize_to_char(entry), usize_to_char(encoding));
    }

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
        usize_to_char(rotor.encode_rtl(0)));
    
    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encode_rtl(0)));
    
    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encode_rtl(0)));

    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encode_rtl(0)));

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
        usize_to_char(rotor.encode_rtl(0)));
    
    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encode_rtl(0)));
    
    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encode_rtl(0)));

    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encode_rtl(0)));

}


// Need to find a step by step encryption example
#[test]
fn settings() {
    let plugboard = Plugboard::new(vec![('A','B'),('X','Z')]);
    let rotor1 = ROTOR_I.clone();
    let rotor2 = ROTOR_II.clone();
    let rotor3 = ROTOR_III.clone();
    let reflector = REFLECTOR_A.clone();

    let rotors = (rotor1, rotor2, rotor3);
    let ring_positions = (0,0,0);

    let mut s = Settings::new( plugboard, rotors, reflector, ring_positions );

    let text = "AAA".chars();
    for c in text {
        print!("{}",s.encode_char(c));
        s.advance_rotors();
    }
    println!("");

    println!("we should get:\nEKV")
}

#[test]
fn settings_2() {
    let plugboard = Plugboard::new(vec![('Z','Z')]);
    let rotor1 = ROTOR_I.clone();
    let rotor2 = ROTOR_II.clone();
    let rotor3 = ROTOR_III.clone();
    let reflector = REFLECTOR_A.clone();

    let rotors = (rotor1, rotor2, rotor3);
    let ring_positions = (0,0,0);

    let mut s = Settings::new( plugboard, rotors, reflector, ring_positions );

    let text = "ENIGMA".chars();
    for c in text {
        print!("{}",s.encode_char(c));
        s.advance_rotors();
    }
    println!("");

    println!("we should get:\nBYEJNJ")
}