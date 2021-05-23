use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};

lazy_static! {

    // Ignoring yoon for now
    pub static ref HIRAGANA: &'static str = "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわをんがぎぐげござじずぜぞだぢづでどばびぶべぼぱぴぷぺぽ";

    pub static ref ROMAN: [&'static str; 71] = ["a", "i", "u", "e", "o", 
                                                "ka", "ki", "ku", "ke", "ko", 
                                                "sa", "shi", "su", "se", "so", 
                                                "ta", "chi", "tsu", "te", "to", 
                                                "na", "ni", "nu", "ne", "no", 
                                                "ha", "hi", "fu", "he", "ho", 
                                                "ma", "mi", "mu", "me", "mo",
                                                "ya", "yu", "yo", 
                                                "ra", "ri", "ru", "re", "ro", 
                                                "wa", "o", 
                                                "n",
                                                "ga", "gi", "gu", "ge", "go",
                                                "za", "ji", "zu", "ze", "zo",
                                                "do", "ji", "zu", "de", "do",
                                                "ba", "bi", "bu", "be", "bo",
                                                "pa", "pi", "pu", "pe", "po"];

    pub static ref HEPBURN_H_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        for (l,w) in HIRAGANA.chars().zip(ROMAN.iter()) {
            println!("{} {}",l,w);
            m.insert(l,*w);
        }
        m
    };

}

// Japanese does not encode reversibly into romaji unless gramatical information is taken into account
pub struct Hepburn<'a> {
    map: HashMap<char, &'a str>,
    syllabary: &'a str,
}

impl Hepburn<'_> {

    pub fn new<'a>() -> Hepburn<'a> {
        Hepburn{ map: HEPBURN_H_MAP.clone(), syllabary: HIRAGANA.clone() }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut symbols = text.chars();
        let mut out = format!("{}",self.map[&symbols.next().unwrap()]);
        for s in symbols {
            out.push_str(&self.map[&s])
        }
        out
    }

}

impl fmt::Display for Hepburn<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hepburn Romanization of Hiragana")
    }

}



#[test]
fn hepburn_hiragana() {
    let hep = Hepburn::new();
    let plaintext = "ひらがな";
    let coded = hep.encode(plaintext);

    assert_eq!(coded,"hiragana");

}
