use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};
use crate::alphabets::LATIN36;

lazy_static! {
    pub static ref NATO_MAP: HashMap<char, String> = {
        let mut m = HashMap::new();
        // Yes, ALFA and JULIETT are meant to be spelled that way
        let words = ["ALFA", "BRAVO", "CHARLIE", "DELTA", "ECHO", "FOXTROT",
                     "GOLF", "HOTEL", "INDIA", "JULIETT", "KILO", "LIMA",
                     "MIKE", "NOVEMBER", "OSCAR", "PAPA", "QUEBEC", "ROMEO",
                     "SIERRA", "TANGO", "UNIFORM", "VICTOR", "WHISKEY",
                     "XRAY", "YANKEE", "ZULU", "ZERO", "ONE", "TWO", "THREE", "FOUR",
                     "FIVE", "SIX", "SEVEN", "EIGHT", "NINE"].iter();
        for (l,w) in LATIN36.chars().zip(words) {
            m.insert(l, w.to_string());
        }
        m
    };

    pub static ref NATO_MAP_INV: HashMap<String, char> = {
        let mut m = HashMap::new();
        // Yes, ALFA and JULIETT are meant to be spelled that way
        let words = ["ALFA", "BRAVO", "CHARLIE", "DELTA", "ECHO", "FOXTROT",
                     "GOLF", "HOTEL", "INDIA", "JULIETT", "KILO", "LIMA",
                     "MIKE", "NOVEMBER", "OSCAR", "PAPA", "QUEBEC", "ROMEO",
                     "SIERRA", "TANGO", "UNIFORM", "VICTOR", "WHISKEY",
                     "XRAY", "YANKEE", "ZULU", "ZERO", "ONE", "TWO", "THREE", "FOUR",
                     "FIVE", "SIX", "SEVEN", "EIGHT", "NINE"].iter();
        for (l,w) in LATIN36.chars().zip(words) {
            m.insert(w.to_string(),l);
        }
        m
    };



    pub static ref CCB_MAP: HashMap<char, String> = {
        let mut m = HashMap::new();
        let words = ["ABLE", "BAKER", "CHARLIE", "DOG", "EASY", "FOX",
                     "GEORGE", "HOW", "ITEM", "JIG", "KING", "LOVE",
                     "MIKE", "NAN", "OBOE", "PETER", "QUEEN", "ROGER",
                     "SUGAR", "TARE", "UNCLE", "VICTOR", "WILLIAM",
                     "XRAY", "YOKE", "ZEBRA", "ZERO", "ONE", "TWO", "THREE", "FOUR",
                     "FIVE", "SIX", "SEVEN", "EIGHT", "NINE"].iter();
        for (l,w) in LATIN36.chars().zip(words) {
            m.insert(l,w.to_string());
        }
        m
    };

    pub static ref CCB_MAP_INV: HashMap<String, char> = {
        let mut m = HashMap::new();
        let words = ["ABLE", "BAKER", "CHARLIE", "DOG", "EASY", "FOX",
                     "GEORGE", "HOW", "ITEM", "JIG", "KING", "LOVE",
                     "MIKE", "NAN", "OBOE", "PETER", "QUEEN", "ROGER",
                     "SUGAR", "TARE", "UNCLE", "VICTOR", "WILLIAM",
                     "XRAY", "YOKE", "ZEBRA", "ZERO", "ONE", "TWO", "THREE", "FOUR",
                     "FIVE", "SIX", "SEVEN", "EIGHT", "NINE"].iter();
        for (l,w) in LATIN36.chars().zip(words) {
            m.insert(w.to_string(),l);
        }
        m
    };





/*     pub static ref RUSSIAN_MAP: HashMap<char, String> = {
        let mut m = HashMap::new();
        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        // Yes, ALFA and JULIETT are meant to be spelled that way
        let words = ["АННА", "БОРИС", "ВАСИЛИЙ", "ГРИГОРИЙ", "ДМИТРИЙ", "ЕЛЕНА",
                     "GEORGE", "HOW", "ITEM", "JIG", "KING", "LOVE",
                     "MIKE", "NAN", "OBOE", "PETER", "QUEEN", "ROGER",
                     "SUGAR", "TARE", "UNCLE", "VICTOR", "WILLIAM",
                     "XRAY", "YOKE", "ZEBRA", "ZERO", "ONE", "TWO", "THREE", "FOUR",
                     "FIVE", "SIX", "SEVEN", "EIGHT", "NINE"].iter();
        for (l,w) in letters.chars().zip(words) {
            m.insert(l,w.to_string());
        }
        m
    }; */
}

pub struct Spelling {
    map: HashMap<char, String>,
    map_inv: HashMap<String, char>,
    alphabet: String,
}

impl Spelling {

    pub fn new(alphabet: &str, words: Vec<&str>) -> Spelling {
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (l,w) in alphabet.chars().zip(words) {
            map.insert(l,w.to_string() );
            map_inv.insert(w.to_string(), l);
        }
        Spelling{ map, map_inv, alphabet: alphabet.to_string() }
        
    }

    pub fn nato() -> Spelling {
        Spelling{ map: NATO_MAP.clone(), map_inv: NATO_MAP_INV.clone(), alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string() }
    }

    pub fn ccb() -> Spelling {
        Spelling{ map: CCB_MAP.clone(), map_inv: CCB_MAP_INV.clone(), alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string() }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut symbols = text.chars();
        let mut out = format!("{}",self.map[&symbols.next().unwrap()]);
        for s in symbols {
            out.push(' ');
            out.push_str(&self.map[&s])
        }
        out
    }

    pub fn encode_nospace(&self, text: &str) -> String {
        let mut out = "".to_string();
        for s in text.chars() {
            out.push_str(&self.map[&s])
        }
        out
    }

    pub fn decode(&self, text: &str) -> String {
        text.split(" ").map(|x| self.map_inv[x]).collect::<String>()
    }

    // Only work if the code words have the unique prefix property
    pub fn decode_nospace(&self, text: &str) -> String {
        let mut output = "".to_string();
        let mut buffer = "".to_string();
        for b in text.chars() {
            buffer.push(b);
            if self.map_inv.contains_key(&buffer) {
                output.push(self.map_inv[&buffer]);
                buffer = "".to_string();
            }
        }
        output
    }
}

impl fmt::Display for Spelling {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "".to_string();
        for c in self.alphabet.chars() {
            s.push_str(&format!("{}: {}\n",c,self.map[&c]))
        }
        write!(f, "Spelling Alphabet\n{}",s)
    }
}


#[test]
fn nato_alphabet() {
    let nato = Spelling::nato();
    let plaintext = "ABC123";
    let coded = nato.encode(plaintext);
    let decoded = nato.decode(&coded);

    assert_eq!(coded,"ALFA BRAVO CHARLIE ONE TWO THREE");
    assert_eq!(decoded, "ABC123");
}

#[test]
fn nato_alphabet_nospace() {
    let nato = Spelling::nato();
    let plaintext = "ABC123";
    let coded = nato.encode_nospace(plaintext);
    let decoded = nato.decode_nospace(&coded);

    assert_eq!(coded,"ALFABRAVOCHARLIEONETWOTHREE");
    assert_eq!(decoded, "ABC123");
}

#[test]
fn ccb_alphabet() {
    let ccb = Spelling::ccb();
    let plaintext = "ABC123";
    let coded = ccb.encode(plaintext);
    let decoded = ccb.decode(&coded);

    assert_eq!(coded,"ABLE BAKER CHARLIE ONE TWO THREE");
    assert_eq!(decoded, "ABC123");
}

#[test]
fn ccb_alphabet_nospace() {
    let ccb = Spelling::ccb();
    let plaintext = "ABC123";
    let coded = ccb.encode_nospace(plaintext);
    let decoded = ccb.decode_nospace(&coded);

    assert_eq!(coded,"ABLEBAKERCHARLIEONETWOTHREE");
    assert_eq!(decoded, "ABC123");
}

