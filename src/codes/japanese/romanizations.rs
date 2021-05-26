use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};

lazy_static! {

    /* 
    There are numerous romanization of Japanese words. The most common, intended to be read by English speakers, in the Hepburn romanization which represents each kana as its
    closest equivalent English syllable. However the Hepburn romanization and may others are not reversible as some kana are pronounced the same and others change pronunciation
    depending on their usage. There are also numerous irregularities. The Nihon-shiki system (日本式ローマ字, nihon-shiki romaji, lit: japanese style roman characters) is a highly
    regularized encoding from kana to the Latin alphabet that gives the 'vowel' kana a single letter, the 'n' kana a single letter, and all other exactly two letters. Every row 
    is defined by a single consonant. It is the only commonly used system that is reversible, encoding from kana to romaji and from romaji to kana without losing information.

    This encoding is not always easy to pronounce for non-natives. For instance こんにちは is "kon'nichiwa" in the Hepburn system but is "kon'tiha" in the Nihon-shiki system. However 
    a variation ont it called Kunrei-shiki (訓令式ローマ字, kunrei-shiki romaji, lit: instructional style roman characters) is the official standard romanization of Japanese with only
    the addition of changes depending on grammatical use of kana.
    */
    pub static ref HIRAGANA: &'static str = "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわをんがぎぐげござじずぜぞだぢづでどばびぶべぼぱぴぷぺぽゃゅょ";

    pub static ref KATAKANA: &'static str = "アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲンガギグゲゴザジズゼゾダヂヅデドバビブベボパピプペポャュョ";


    pub static ref ROMAN: [&'static str; 74] = [ "a",  "i",  "u",  "e",  "o", 
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
                                                "pa", "pi", "pu", "pe", "po",
                                                "ya",       "yu",       "yo"];


    pub static ref ROMAN_H_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        for (kana,syll) in HIRAGANA.chars().zip(ROMAN.iter()) {
            m.insert(kana,*syll);
        }
        m
    };

    pub static ref ROMAN_K_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        for (kana,syll) in KATAKANA.chars().zip(ROMAN.iter()) {
            m.insert(kana,*syll);
        }
        m
    };

}


pub struct NihonShikiHiragana<'a> {
    map: HashMap<char, &'a str>,
    syllabary: &'a str,
}

impl NihonShikiHiragana<'_> {

    pub fn new<'a>() -> NihonShikiHiragana<'a> {
        NihonShikiHiragana{ map: ROMAN_H_MAP.clone(), syllabary: HIRAGANA.clone() }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut symbols = text.chars().peekable();
        let mut out = String::new();
        // Japanese doesn't have vowels but these characters begin with a vowel when romanized
        let vowels = ['あ','い','う','え','お','や','ゆ','よ'];
        // kana start start with n when romanized
        let n_kana = ['な','に','ぬ','ね','の'];

        // The small y-kana
        let small_y = ['ゃ', 'ゅ', 'ょ'];

        loop {
            let s = match symbols.next() {
                Some(kana) => kana,
                None => break,
            };
            if s.is_whitespace() {
                out.push(s);
            // handle apostophe after ん
            } else if s == 'ん' {
                let next_kana = symbols.peek();
                if next_kana.is_none() {
                    out.push('n')
                } else {
                    let k = next_kana.unwrap();
                    if vowels.contains(k) || n_kana.contains(k) {
                        out.push_str("n'");
                    } else {
                        out.push('n');
                    }
                }
            // handle sokuon 
            } else if s == 'っ' {
                let next_kana = symbols.peek().unwrap();
                let romaji = self.map[next_kana].chars().nth(0).unwrap();
                out.push(romaji);
            // handle yoon
            } else if small_y.contains(&s) {
                let prev_char = out.pop().unwrap();
                if prev_char == 'i' {
                    out.push_str(&self.map[&s])
                } else {
                    panic!("small y kana must be preceeded by a i-column kana")
                }
            // everything else
            } else {
                out.push_str(&self.map[&s])
            }

        }
        out
    }

    pub fn char_map(&self) -> String {
        let mut out = String::new();
        for c in self.syllabary.chars() {
            out.push_str(&format!("{}  {}\n",c,self.map[&c]))
        }
        out
    }

}

impl fmt::Display for NihonShikiHiragana<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nihon-shiki Romanization of Hiragana")
    }

}


pub struct NihonShikiKatakana<'a> {
    map: HashMap<char, &'a str>,
    syllabary: &'a str,
}

impl NihonShikiKatakana<'_> {

    pub fn new<'a>() -> NihonShikiKatakana<'a> {
        NihonShikiKatakana{ map: ROMAN_K_MAP.clone(), syllabary: KATAKANA.clone() }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut symbols = text.chars().peekable();
        let mut out = String::new();

        // Japanese doesn't have vowels but these characters begin with a vowel when romanized
        let vowels = ['ア','イ','ウ','エ','オ','ユ','ヨ','ラ'];
        // kana start start with n when romanized
        let n_kana = ['ナ','ニ','ヌ','ネ','ノ'];
        // The small y-kana
        let small_y = ['ャ','ュ','ョ'];

        loop {
            let s = match symbols.next() {
                Some(kana) => kana,
                None => break,
            };
            if s.is_whitespace() {
                out.push(' ');
            // handle apostophe after ン
            } else if s == 'ン' {
                let next_kana = symbols.peek();
                if next_kana.is_none() {
                    out.push('n')
                } else {
                    let k = next_kana.unwrap();
                    if vowels.contains(k) || n_kana.contains(k) {
                        out.push_str("n'");
                    } else {
                        out.push('n');
                    }
                }
            // handle chōonpu 
            } else if s == 'ー' {
                let vowel = out.pop().unwrap();
                out.push(vowel);
                out.push(vowel);
            
            // handle sokuon 
            } else if s == 'ッ' {
                let next_kana = symbols.peek().unwrap();
                let romaji = self.map[next_kana].chars().nth(0).unwrap();
                out.push(romaji);
            // handle yoon
            } else if small_y.contains(&s) {
                let prev_char = out.pop().unwrap();
                if prev_char == 'i' {
                    out.push_str(&self.map[&s])
                } else {
                    panic!("small y kana must be preceeded by a i-column kana")
                }
            // everything else
            } else {
                out.push_str(&self.map[&s])
            }

        }
        out
    }

    pub fn char_map(&self) -> String {
        let mut out = String::new();
        for c in self.syllabary.chars() {
            out.push_str(&format!("{}  {}\n",c,self.map[&c]))
        }
        out
    }

}

impl fmt::Display for NihonShikiKatakana<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nihon-shiki Romanization of Katakana")
    }

}


#[test]
fn nihon_shiki_hiragana() {
    let ns = NihonShikiHiragana::new();
    let plaintext = "ひらがな かたかな しんよう きっぷ きよう きょう にほん　こんにちは";
    let coded = ns.encode(plaintext);

    //println!("{}",ns.char_map());

    println!("{}",coded);

}


#[test]
fn nihon_shiki_katakana() {
    let ns = NihonShikiKatakana::new();
    let plaintext = "ラーメン";
    let coded = ns.encode(plaintext);

    //println!("{}",ns.char_map());

    println!("{}",coded);

}
