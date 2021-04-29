use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref BACON_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let codes = ["00000","00001","00010","00011","00100","00101","00110","00111",
                     "01000","01001","01010","01011","01100","01101","01110","01111",
                     "10000","10001","10010","10011","10100","10101","10110","10111",
                     "11000","11001"];
        for (l,c) in letters.chars().zip(codes.iter()) {
            m.insert(l, *c);
        }
        m
    };

    pub static ref BACON_MAP_INV: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let codes = ["00000","00001","00010","00011","00100","00101","00110","00111",
                     "01000","01001","01010","01011","01100","01101","01110","01111",
                     "10000","10001","10010","10011","10100","10101","10110","10111",
                     "11000","11001"];
        for (l,c) in letters.chars().zip(codes.iter()) {
            m.insert(*c,l);
        }
        m
    };

}

pub struct Bacon {
    map: HashMap<char, &'static str>,
    map_inv: HashMap<&'static str, char>,
}

impl Bacon {

    pub fn new() -> Bacon {
        Bacon{ map: BACON_MAP.clone(), map_inv: BACON_MAP_INV.clone() }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut out = "".to_string();
        for s in text.chars() {
            out.push_str(self.map[&s])
        }
        out
    }

    pub fn decode(&self, text: &str) -> String {
        let mut out = "".to_string();
        for p in 0..(text.len()/5) {
            let group = &text[(p*5)..(p*5)+5];
            out.push(self.map_inv[group])
        }
        out
    }
}

#[test]
fn bacon() {
    let bacon = Bacon::new();
    let plaintext = "THEQUICK";
    let coded = bacon.encode(plaintext);
    let decoded = bacon.decode(&coded);

    println!("{}",plaintext);
    println!("{}",coded);
    println!("{}",decoded);

    
}