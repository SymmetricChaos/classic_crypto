use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref ITU_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        let letters = " ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
        let codes = ["0",
                     "10111", // A
                     "111010101", // B
                     "11101011101", // C
                     "1110101", // D
                     "1", // E
                     "101011101", // F
                     "111011101", // G
                     "1010101", // H
                     "101", // I
                     "1011101110111", // J
                     "111010111", // K
                     "101110101", // L
                     "1110111", // M
                     "11101", // N
                     "11101110111", // O
                     "10111011101", // P
                     "1110111010111", // Q
                     "1011101", // R
                     "10101", // S
                     "111", // T
                     "1010111", // U
                     "101010111", // V
                     "101110111", // W
                     "11101010111", // X
                     "1110101110111", // Y
                     "11101110101", // Z
                     "10111011101110111",
                     "101011101110111",
                     "1010101110111",
                     "10101010111",
                     "101010101",
                     "11101010101",
                     "1110111010101",
                     "111011101110101",
                     "11101110111011101",
                     "1110111011101110111"];
        for (l,c) in letters.chars().zip(codes.iter()) {
            m.insert(l, *c);
        }
        m
    };

    pub static ref ITU_MAP_INV: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        let letters = " ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
        let codes = ["0",
                     "10111", // A
                     "111010101", // B
                     "11101011101", // C
                     "1110101", // D
                     "1", // E
                     "101011101", // F
                     "111011101", // G
                     "1010101", // H
                     "101", // I
                     "1011101110111", // J
                     "111010111", // K
                     "101110101", // L
                     "1110111", // M
                     "11101", // N
                     "11101110111", // O
                     "10111011101", // P
                     "1110111010111", // Q
                     "1011101", // R
                     "10101", // S
                     "111", // T
                     "1010111", // U
                     "101010111", // V
                     "101110111", // W
                     "11101010111", // X
                     "1110101110111", // Y
                     "11101110101", // Z
                     "10111011101110111",
                     "101011101110111",
                     "1010101110111",
                     "10101010111",
                     "101010101",
                     "11101010101",
                     "1110111010101",
                     "111011101110101",
                     "11101110111011101",
                     "1110111011101110111"];
        for (l,c) in letters.chars().zip(codes.iter()) {
            m.insert(*c, l);
        }
        m
    };
}

pub struct MorseITU {
    map: HashMap<char, &'static str>,
    map_inv: HashMap<&'static str, char>,
}

impl MorseITU {

    pub fn default() -> MorseITU {
        MorseITU{ map: ITU_MAP.clone(), map_inv: ITU_MAP_INV.clone() }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut out = Vec::new();
        for s in text.chars() {
            out.push(self.map[&s])
        }
        out.join("00")
    }

    pub fn decode(&self, text: &str) -> String {
        text.split("00").map(|x| self.map_inv[x]).collect::<String>()
    }
}

#[test]
fn morse_itu() {
    let itu = MorseITU::default();
    let plaintext = "THEQUICK";
    let coded = itu.encode(plaintext);
    let decoded = itu.decode(&coded);

    println!("{}",plaintext);
    println!("{}",coded);
    println!("{}",decoded);
}