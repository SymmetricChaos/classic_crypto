use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};
use crate::alphabets::LATIN36;

lazy_static! {
    // Yes, ALFA and JULIETT are meant to be spelled that way
    pub static ref NATO: [&'static str; 36] = ["ALFA", "BRAVO", "CHARLIE", "DELTA", "ECHO", "FOXTROT",
                                                "GOLF", "HOTEL", "INDIA", "JULIETT", "KILO", "LIMA",
                                                "MIKE", "NOVEMBER", "OSCAR", "PAPA", "QUEBEC", "ROMEO",
                                                "SIERRA", "TANGO", "UNIFORM", "VICTOR", "WHISKEY",
                                                "XRAY", "YANKEE", "ZULU", "ZERO", "ONE", "TWO", "THREE", "FOUR",
                                                "FIVE", "SIX", "SEVEN", "EIGHT", "NINE"];

    pub static ref NATO_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        for (l,w) in LATIN36.chars().zip( NATO.iter()) {
            m.insert(l, *w);
        }
        m
    };

    pub static ref NATO_MAP_INV: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        // Yes, ALFA and JULIETT are meant to be spelled that way
        for (l,w) in LATIN36.chars().zip(NATO.iter()) {
            m.insert(*w,l);
        }
        m
    };



    pub static ref CCB: [&'static str; 36] = ["ABLE", "BAKER", "CHARLIE", "DOG", "EASY", "FOX",
                                              "GEORGE", "HOW", "ITEM", "JIG", "KING", "LOVE",
                                              "MIKE", "NAN", "OBOE", "PETER", "QUEEN", "ROGER",
                                              "SUGAR", "TARE", "UNCLE", "VICTOR", "WILLIAM",
                                              "XRAY", "YOKE", "ZEBRA", "ZERO", "ONE", "TWO", "THREE", "FOUR",
                                              "FIVE", "SIX", "SEVEN", "EIGHT", "NINE"];

    pub static ref CCB_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        for (l,w) in LATIN36.chars().zip(CCB.iter()) {
            m.insert(l,*w);
        }
        m
    };

    pub static ref CCB_MAP_INV: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        for (l,w) in LATIN36.chars().zip(CCB.iter()) {
            m.insert(*w,l);
        }
        m
    };

}

pub struct SpellingAlphabet<'a> {
    map: HashMap<char, &'a str>,
    map_inv: HashMap<&'a str, char>,
    alphabet: &'a str,
}

impl SpellingAlphabet<'_> {

    pub fn new<'a>(alphabet: &'a str, words: Vec<&'a str>) -> SpellingAlphabet<'a> {
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (l,w) in alphabet.chars().zip(words) {
            map.insert(l,w );
            map_inv.insert(w, l);
        }
        SpellingAlphabet{ map, map_inv, alphabet }
        
    }

    pub fn nato() -> SpellingAlphabet<'static> {
        SpellingAlphabet{ map: NATO_MAP.clone(), map_inv: NATO_MAP_INV.clone(), alphabet: LATIN36 }
    }

    pub fn ccb() -> SpellingAlphabet<'static> {
        SpellingAlphabet{ map: CCB_MAP.clone(), map_inv: CCB_MAP_INV.clone(), alphabet: LATIN36 }
    }



}

impl fmt::Display for SpellingAlphabet<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Spelling Alphabet")
    }
}

impl crate::Code for SpellingAlphabet<'_> {
    fn encode(&self, text: &str) -> String {
        let mut symbols = text.chars();
        let mut out = format!("{}",self.map[&symbols.next().unwrap()]);
        for s in symbols {
            out.push(' ');
            out.push_str(&self.map[&s])
        }
        out
    }

    fn decode(&self, text: &str) -> String {
        text.split(" ").map(|x| self.map_inv[x]).collect::<String>()
    }

    fn char_map(&self) -> String {
        let mut out = String::new();
        for l in self.alphabet.chars() {
            out.push_str(&format!("{}  {}\n", l,self.map[&l]))
        }
        out
    }
}



#[test]
fn nato_alphabet() {
    use crate::Code;
    let nato = SpellingAlphabet::nato();
    let plaintext = "ABC123";
    let coded = nato.encode(plaintext);
    let decoded = nato.decode(&coded);

    assert_eq!(coded,"ALFA BRAVO CHARLIE ONE TWO THREE");
    assert_eq!(decoded, "ABC123");
}

#[test]
fn ccb_alphabet() {
    use crate::Code;
    let ccb = SpellingAlphabet::ccb();
    let plaintext = "ABC123";
    let coded = ccb.encode(plaintext);
    let decoded = ccb.decode(&coded);

    assert_eq!(coded,"ABLE BAKER CHARLIE ONE TWO THREE");
    assert_eq!(decoded, "ABC123");
}