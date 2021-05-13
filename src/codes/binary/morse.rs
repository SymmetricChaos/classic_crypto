use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref LETTERS: &'static str = " ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
    pub static ref CODES: [&'static str; 37] = ["0", "10111", "111010101", "11101011101", "1110101", "1", "101011101", "111011101", "1010101", "101", "1011101110111", 
                                                "111010111", "101110101", "1110111", "11101", "11101110111", "10111011101", "1110111010111", "1011101", "10101", "111", 
                                                "1010111", "101010111", "101110111", "11101010111", "1110101110111", "11101110101", "10111011101110111", "101011101110111", 
                                                "1010101110111", "10101010111", "101010101", "11101010101", "1110111010101", "111011101110101", "11101110111011101", 
                                                "1110111011101110111"];
    pub static ref ITU_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        for (l,c) in LETTERS.chars().zip(CODES.iter()) {
            m.insert(l, *c);
        }
        m
    };

    pub static ref ITU_MAP_INV: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        for (l,c) in LETTERS.chars().zip(CODES.iter()) {
            m.insert(*c, l);
        }
        m
    };
}

pub struct MorseITU {
    map: HashMap<char, &'static str>,
    map_inv: HashMap<&'static str, char>,
    alphabet: &'static str,
}

impl MorseITU {

    pub fn default() -> MorseITU {
        MorseITU{ map: ITU_MAP.clone(), map_inv: ITU_MAP_INV.clone(), alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890" }
    }

}

impl crate::Code for MorseITU {
    fn encode(&self, text: &str) -> String {
        let mut out = Vec::new();
        for s in text.chars() {
            out.push(self.map[&s])
        }
        out.join("00")
    }

    fn decode(&self, text: &str) -> String {
        text.split("00").map(|x| self.map_inv[x]).collect::<String>()
    }

    fn char_map(&self) -> String {
        let mut out = String::new();
        for l in self.alphabet.chars() {
            out.push_str(&format!("{}  {}\n",l,self.map[&l]))
        }
        out.push_str("\n[symbol space]   00\n[word space]     00000\n");
        out
    }
}