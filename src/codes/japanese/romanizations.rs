use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};

lazy_static! {

    // Ignoring yoon for now
    pub static ref HIRAGANA: &'static str = "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわをんがぎぐげござじずぜぞだぢづでどばびぶべぼぱぴぷぺぽ";

    pub static ref HEPBURN: [&'static str; 71] = [ "a",  "i",   "u",   "e",  "o", 
                                                  "ka", "ki",  "ku",  "ke", "ko", 
                                                  "sa", "shi", "su",  "se", "so", 
                                                  "ta", "chi", "tsu", "te", "to", 
                                                  "na", "ni",  "nu",  "ne", "no", 
                                                  "ha", "hi",  "fu",  "he", "ho", 
                                                  "ma", "mi",  "mu",  "me", "mo",
                                                  "ya",        "yu",        "yo", 
                                                  "ra", "ri",  "ru",  "re", "ro", 
                                                  "wa",                      "o", // we assume that を is always o and never wo for simplicity
                                                  "n",
                                                  "ga", "gi",  "gu",  "ge", "go",
                                                  "za", "ji",  "zu",  "ze", "zo",
                                                  "do", "ji",  "zu",  "de", "do",
                                                  "ba", "bi",  "bu",  "be", "bo",
                                                  "pa", "pi",  "pu",  "pe", "po"];

    

    pub static ref HEPBURN_H_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        for (kana,syll) in HIRAGANA.chars().zip(HEPBURN.iter()) {
            m.insert(kana,*syll);
        }
        m
    };

}

// Japanese does not encode reversibly into romaji unless gramatical information is taken into account
// Interestingly he Hepburn romanization is fairly irregular (a single kana may be a 1, 2, or 3 letters) making it very useful as part of a transposition cipher
pub struct Hepburn<'a> {
    map: HashMap<char, &'a str>,
    syllabary: &'a str,
}

impl Hepburn<'_> {

    pub fn new<'a>() -> Hepburn<'a> {
        Hepburn{ map: HEPBURN_H_MAP.clone(), syllabary: HIRAGANA.clone() }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut symbols = text.chars().peekable();
        let mut out = format!("{}",self.map[&symbols.next().unwrap()]);
        // Japanese doesn't have vowels but these characters begin with a vowel when romanized
        let vowels = ['あ','い','う','え','お','や','ゆ','よ','を'];


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

impl fmt::Display for Hepburn<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hepburn Romanization of Hiragana")
    }

}



#[test]
fn hepburn_hiragana() {
    let hep = Hepburn::new();
    let plaintext = "ひらがな かたかな しんよう きっぷ";
    let coded = hep.encode(plaintext);

    println!("{}",coded);

}
