use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};


lazy_static! {
    pub static ref NATO_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        // Yes, ALFA and JULIETT are meant to be spelled that way
        let words = ["ALFA", "BRAVO", "CHARLIE", "DELTA", "ECHO", "FOXTROT",
                     "GOLF", "HOTEL", "INDIA", "JULIETT", "KILO", "LIMA",
                     "MIKE", "NOVEMBER", "OSCAR", "PAPA", "QUEBEC", "ROMEO",
                     "SIERRA", "TANGO", "UNIFORM", "VICTOR", "WHISKEY",
                     "XRAY", "YANKEE", "ZULU", "ZERO", "ONE", "TWO", "THREE", "FOUR",
                     "FIVE", "SIX", "SEVEN", "EIGHT", "NINE"].iter();
        for (l,w) in letters.chars().zip(words) {
            m.insert(l, *w);
        }
        m
    };

    pub static ref NATO_MAP_INV: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        // Yes, ALFA and JULIETT are meant to be spelled that way
        let words = ["ALFA", "BRAVO", "CHARLIE", "DELTA", "ECHO", "FOXTROT",
                     "GOLF", "HOTEL", "INDIA", "JULIETT", "KILO", "LIMA",
                     "MIKE", "NOVEMBER", "OSCAR", "PAPA", "QUEBEC", "ROMEO",
                     "SIERRA", "TANGO", "UNIFORM", "VICTOR", "WHISKEY",
                     "XRAY", "YANKEE", "ZULU", "ZERO", "ONE", "TWO", "THREE", "FOUR",
                     "FIVE", "SIX", "SEVEN", "EIGHT", "NINE"].iter();
        for (l,w) in letters.chars().zip(words) {
            m.insert(*w,l);
        }
        m
    };



    pub static ref CCB_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        // Yes, ALFA and JULIETT are meant to be spelled that way
        let words = ["ABLE", "BAKER", "CHARLIE", "DOG", "EASY", "FOX",
                     "GEORGE", "HOW", "ITEM", "JIG", "KING", "LOVE",
                     "MIKE", "NAN", "OBOE", "PETER", "QUEEN", "ROGER",
                     "SUGAR", "TARE", "UNCLE", "VICTOR", "WILLIAM",
                     "XRAY", "YOKE", "ZEBRA", "ZERO", "ONE", "TWO", "THREE", "FOUR",
                     "FIVE", "SIX", "SEVEN", "EIGHT", "NINE"].iter();
        for (l,w) in letters.chars().zip(words) {
            m.insert(l,*w);
        }
        m
    };

    pub static ref CCB_MAP_INV: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        // Yes, ALFA and JULIETT are meant to be spelled that way
        let words = ["ABLE", "BAKER", "CHARLIE", "DOG", "EASY", "FOX",
                     "GEORGE", "HOW", "ITEM", "JIG", "KING", "LOVE",
                     "MIKE", "NAN", "OBOE", "PETER", "QUEEN", "ROGER",
                     "SUGAR", "TARE", "UNCLE", "VICTOR", "WILLIAM",
                     "XRAY", "YOKE", "ZEBRA", "ZERO", "ONE", "TWO", "THREE", "FOUR",
                     "FIVE", "SIX", "SEVEN", "EIGHT", "NINE"].iter();
        for (l,w) in letters.chars().zip(words) {
            m.insert(*w,l);
        }
        m
    };
}

pub struct Spelling {
    map: HashMap<char, &'static str>,
    map_inv: HashMap<&'static str, char>,
    alphabet: String,
}

impl Spelling {

/*     pub fn new(alphabet: &str, words: Vec<String>) -> Spelling {
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (l,w) in alphabet.chars().zip(words) {
            map.insert(l,w.clone() );
            map_inv.insert(w, l);
        }
        Spelling{ map, map_inv, alphabet: alphabet.to_string() }
        
    } */

    pub fn nato() -> Spelling {
        Spelling{ map: NATO_MAP.clone(), map_inv: NATO_MAP_INV.clone(), alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string() }
    }

    pub fn ccb() -> Spelling {
        Spelling{ map: CCB_MAP.clone(), map_inv: CCB_MAP_INV.clone(), alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string() }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut out = Vec::new();
        for s in text.chars() {
            out.push(self.map[&s])
        }
        out.join(" ")
    }

    pub fn decode(&self, text: &str) -> String {
        text.split(" ").map(|x| self.map_inv[x]).collect::<String>()
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

    //println!("{}",nato);

    println!("{}",plaintext);
    println!("{}",coded);
    println!("{}",decoded);
}

#[test]
fn ccb_alphabet() {
    let ccb = Spelling::ccb();
    let plaintext = "ABC123";
    let coded = ccb.encode(plaintext);
    let decoded = ccb.decode(&coded);

    //println!("{}",ccb);

    println!("{}",plaintext);
    println!("{}",coded);
    println!("{}",decoded);
}
