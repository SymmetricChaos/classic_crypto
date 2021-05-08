use lazy_static::lazy_static;
use std::{cell::Cell, collections::HashMap};
use super::code_generators::FixedWidthInteger;

// https://www.cryptomuseum.com/ref/ita2/index.htm

// The Unicode Control Pictures: ␎ and ␏ are used to represent the Letter Shift and Figure Shift control characters of the ITA2
// The other control characters (NUL, LF, CR, ENC, BEL) are rendered as their equivalent Unicode Control Pictures except for SP which is the space character
lazy_static! {

    pub static ref LETTERS: &'static str = "␀E␊A SIU␍DRJNFCKTZLWHYPQOBG␎MXV␏";
    pub static ref FIGURES: &'static str = "␀3␊- '87␍␅4␇,!:(5+)2£6019?&␎./;␏";

    pub static ref ITA2_MAP_LETTER: HashMap<char,String> = {
        let mut m = HashMap::new();
        let codes = FixedWidthInteger::new(5);
        for (l,c) in LETTERS.chars().zip(codes) {
            m.insert(l,c);
        }
        m
    };

    // Map figures to codes
    pub static ref ITA2_MAP_FIGURE: HashMap<char,String> = {
        let mut m = HashMap::new();
        let codes = FixedWidthInteger::new(5);
        for (f,c) in FIGURES.chars().zip(codes) {
            m.insert(f,c);
        }
        m
    };

    // Map codes to figures or letters
    pub static ref ITA2_MAP_INV: HashMap<String,(char,char)> = {
        let mut m = HashMap::new();
        let pairs: Vec<(char,char)> = LETTERS.chars().zip(FIGURES.chars()).map(|(a,b)| (a,b)).collect();
        let codes = FixedWidthInteger::new(5);
        for (p,c) in pairs.iter().zip(codes) {
            m.insert(c,*p);
        }
        m
    };

}




pub struct BaudotITA2 {
    map_letter: HashMap<char, String>,
    map_figure: HashMap<char, String>,
    map_inv: HashMap<String,(char,char)>,
    figures: Cell<bool>,
}

impl BaudotITA2 {

    pub fn default() -> BaudotITA2 {
        BaudotITA2{ map_letter: ITA2_MAP_LETTER.clone(), map_figure: ITA2_MAP_FIGURE.clone(), map_inv: ITA2_MAP_INV.clone(), figures: Cell::new(false) }
    }

}

impl crate::Code for BaudotITA2 {
    fn encode(&self, text: &str) -> String {
        self.figures.set(false);
        let mut out = String::new();
        for s in text.chars() {
            if s == '␎' {
                if self.figures.get() == false {
                    self.figures.set(true);
                }
            }
            if s == '␏' {
                if self.figures.get() == true {
                    self.figures.set(false);
                }
            }
            match self.figures.get() {
                false => out.push_str(&self.map_letter[&s]),
                true =>  out.push_str(&self.map_figure[&s]),
            }
            
        }
        self.figures.set(false);
        out
    }

    fn decode(&self, text: &str) -> String {
        self.figures.set(false);
        let mut out = String::new();
        for s in text.chars().collect::<Vec<char>>().chunks(5) {
            let code = format!("{}",s.iter().collect::<String>());
            if code == "11011" {
                if self.figures.get() == false {
                    self.figures.set(true);
                }
            }
            if code == "11111" {
                if self.figures.get() == true {
                    self.figures.set(false);
                }
            }
            match self.figures.get() {
                false => out.push(self.map_inv[&code].0),
                true =>  out.push(self.map_inv[&code].1),
            }
            
        }
        self.figures.set(false);
        out
    }

    fn char_map(&self) -> String {
        let mut out = String::with_capacity(441);
        let pairs: Vec<(char,char)> = LETTERS.chars().zip(FIGURES.chars()).map(|(a,b)| (a,b)).collect();
        let codes = FixedWidthInteger::new(5);
        for ((p1,p2),c) in pairs.iter().zip(codes) {
            out.push_str(&format!("{}  {}   {}\n",p1,p2,c))
        }
        out
    }
}


#[test]
fn baudot_ita2() {
    use crate::auxiliary::Code;
    let ita2 = BaudotITA2::default();
    let plaintext = "THEQUICK␎12345␏BROWNFOX";
    let coded = ita2.encode(plaintext);
    let decoded = ita2.decode(&coded);

    println!("{}",plaintext);
    println!("{}",coded);
    println!("{}",decoded);

}
