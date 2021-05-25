use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};

lazy_static! {

    // Ignoring yoon for now
    pub static ref HIRAGANA: &'static str = "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわをんがぎぐげござじずぜぞだぢづでどばびぶべぼぱぴぷぺぽ";

    pub static ref ROMAN: [&'static str; 71] = [ "a",  "i",  "u",  "e",  "o", 
                                                "ka", "ki", "ku", "ke", "ko", 
                                                "sa", "si", "su", "se", "so", 
                                                "ta", "ti", "tu", "te", "to", 
                                                "na", "ni", "nu", "ne", "no", 
                                                "ha", "hi", "hu", "he", "ho", 
                                                "ma", "mi", "mu", "me", "mo",
                                                "ya",       "yu",       "yo", 
                                                "ra", "ri", "ru", "re", "ro", 
                                                "wa",                   "wo",
                                                "n",
                                                "ga", "gi", "gu", "ge", "go",
                                                "za", "zi", "zu", "ze", "zo",
                                                "da", "di", "du", "de", "do",
                                                "ba", "bi", "bu", "be", "bo",
                                                "pa", "pi", "pu", "pe", "po"];

    

    pub static ref ROMAN_H_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        for (kana,syll) in HIRAGANA.chars().zip(ROMAN.iter()) {
            m.insert(kana,*syll);
        }
        m
    };

}

/* 
There is no "good" romanization of hiragana and katana due to characters sharing pronunciations like ぢ and じ as well as kana with irregular pronuciation like ふ and は. 
The Nihon-shiki romanization is a regularized encoding from kana to roman letters so that each kana has a unique representation that is consistent with the rest of its
row in the syllabary.
*/
pub struct NihonShiki<'a> {
    map: HashMap<char, &'a str>,
    syllabary: &'a str,
}

impl NihonShiki<'_> {

    pub fn new<'a>() -> NihonShiki<'a> {
        NihonShiki{ map: ROMAN_H_MAP.clone(), syllabary: HIRAGANA.clone() }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut symbols = text.chars().peekable();
        let mut out = format!("{}",self.map[&symbols.next().unwrap()]);
        // Japanese doesn't have vowels but these characters begin with a vowel when romanized
        let vowels = ['あ','い','う','え','お','や','ゆ','よ'];


        loop {
            let s = match symbols.next() {
                Some(kana) => kana,
                None => break,
            };
            if s.is_whitespace() {
                out.push(s);
            } else if s == 'ん' {
                if vowels.contains(&symbols.peek().unwrap()) {
                    out.push_str("n'");
                } else {
                    out.push('n');
                }
            } else if s == 'っ' {
                let next_kana = symbols.peek().unwrap();
                let romaji = self.map[next_kana].chars().nth(0).unwrap();
                out.push(romaji);
            } else {
                out.push_str(&self.map[&s])
            }

        }
        out
    }

}

impl fmt::Display for NihonShiki<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nihon-shiki Romanization of Hiragana")
    }

}



#[test]
fn hepburn_hiragana() {
    let hep = NihonShiki::new();
    let plaintext = "ひらがな かたかな しんよう きっぷ";
    let coded = hep.encode(plaintext);

    println!("{}",coded);

}
