use lazy_static::lazy_static;
use std::collections::HashMap;
use super::code_generators::FixedWidthInteger;

// https://www.cryptomuseum.com/ref/ita2/index.htm

// Map letters to codes, the # and % symbols were not used by ITA2, here they represents the control codes FIGS and LTRS
// The symbols are dropped during decoding
// The other control characters (NUL, LF, CR, ENC, BEL) are rendered as their Unicode Control Pictures
lazy_static! {

    pub static ref ITA2_MAP_LETTER: HashMap<char,String> = {
        let mut m = HashMap::new();
        let letters = "␀E␊A SIU␍DRJNFCKTZLWHYPQOBG#MXV%";
        let codes = FixedWidthInteger::new(5);
        for (l,c) in letters.chars().zip(codes) {
            m.insert(l,c);
        }
        m
    };

    // Map figures to codes
    pub static ref ITA2_MAP_FIGURE: HashMap<char,String> = {
        let mut m = HashMap::new();
        let figures = "␀3␊- '87␍␅4␇,!:(5+)2$6019?&#./;%";
        let codes = FixedWidthInteger::new(5);
        for (f,c) in figures.chars().zip(codes) {
            m.insert(f,c);
        }
        m
    };

    // Map codes to figures or letters
    pub static ref ITA2_MAP_INV: HashMap<String,(char,char)> = {
        let mut m = HashMap::new();
        let letters = "␀E␊A SIU␍DRJNFCKTZLWHYPQOBG#MXV%";
        let figures = "␀3␊- '87␍␅4␇,!:(5+)2$6019?&#./;%";
        let pairs: Vec<(char,char)> = letters.chars().zip(figures.chars()).map(|(a,b)| (a,b)).collect();
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
    figures: bool,
}

impl BaudotITA2 {

    pub fn default() -> BaudotITA2 {
        BaudotITA2{ map_letter: ITA2_MAP_LETTER.clone(), map_figure: ITA2_MAP_FIGURE.clone(), map_inv: ITA2_MAP_INV.clone(), figures: false }
    }

    pub fn encode(&mut self, text: &str) -> String {
        let mut out = String::new();
        for s in text.chars() {
            if s == '#' {
                if self.figures == false {
                    self.figures = true;
                }
            }
            if s == '%' {
                if self.figures == true {
                    self.figures = false;
                }
            }
            match self.figures {
                false => out.push_str(&self.map_letter[&s]),
                true =>  out.push_str(&self.map_figure[&s]),
            }
            
        }
        out
    }

    pub fn decode(&mut self, text: &str) -> String {
        let mut out = String::new();
        for s in text.chars().collect::<Vec<char>>().chunks(5) {
            let code = format!("{}",s.iter().collect::<String>());
            println!("{}",code);
            if code == "11011" {
                if self.figures == false {
                    self.figures = true;
                }
                continue
            }
            if code == "11111" {
                if self.figures == true {
                    self.figures = false;
                }
                continue
            }
            match self.figures {
                false => out.push(self.map_inv[&code].0),
                true =>  out.push(self.map_inv[&code].1),
            }
            
        }
        out
    }

}

#[test]
fn baudot_ita2() {
    let mut ita2 = BaudotITA2::default();
    let plaintext = "THEQUICK#12345%BROWNFOX";
    let coded = ita2.encode(plaintext);
    let decoded = ita2.decode(&coded);

    println!("{}",plaintext);
    println!("{}",coded);
    println!("{}",decoded);
}
