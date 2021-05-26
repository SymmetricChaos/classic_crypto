//! Common alphabets

use rand::{prelude::SliceRandom, thread_rng};
use itertools::Itertools;

/// The 26 letter Latin alphabet used in English
pub const LATIN26: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// As LATIN26 but with J removed
pub const LATIN25_J: &'static str = "ABCDEFGHIKLMNOPQRSTUVWXYZ";

/// AS LATIN26 but with Q removed
pub const LATIN25_Q: &'static str = "ABCDEFGHIJKLMNOPRSTUVWXYZ";

/// The 26 letter Latin alphabet used in English with the ten digits
pub const LATIN36: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"; 

/// The 23 letter classical Latin alphabet
pub const LATIN23: &'static str = "ABCDEFGHIKLMNOPQRSTVXY";

/// The 26 letter Latin alphabet used in English sorted by approximate frequency
pub const LATIN26_FREQ: &'static str = "ETAOINSHRDLCUMWFGYPBVKJXQZ";

/// The 26 letter Latin alphabet used in English sorted by the order on the QWERTY keyboard
pub const LATIN26_QWERTY: &'static str = "QWERTYUIOPASDFGHJKLZXCVBNM";

/// The 26 letter Latin alphabet used in English sorted by the order on the AZERTY keyboard
pub const LATIN26_AZERTY: &'static str = "AZERTYUIOPQSDFGHJKLMWXCVBN";

/// The 26 letter Latin alphabet used in English sorted by the order on the QWERTZ keyboard
pub const LATIN26_QWERTZ: &'static str = "QWERTZUIOPASDFGHJKLYXCVBNM";

/// The 10 digits from 0 to 9
pub const DIGITS_0: &'static str = "0123456789";

/// The 10 digits from 1 to 0
pub const DIGITS_1: &'static str = "1234567890";

/// The 95 printing ASCII characters
pub const ASCII95: &'static str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

/// As ASCII95 with the space removed
pub const ASCII94: &'static str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~"; 

/// The 128 ASCII characters with control pictures for control codes
pub const ASCII128: &'static str = "␀␁␂␃␄␅␆␇␈␉␊␋␌␍␎␏␐␑␒␓␔␕␖␗␘␙␚␛␜␝␞␟ !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~␡"; 


/// The 24 letter modern Greek alphabet
pub const GREEK24: &'static str = "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ"; 

/// The Russian alphabet
pub const CYRILLIC33: &'static str = "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ"; 

/// The MIME Base64 characters
pub const BASE64: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; 


// Randomized copy of an alphabet
pub fn scramble_alphabet(alphabet: &str) -> String {
    let mut rng = thread_rng();
    let mut v: Vec<char> = alphabet.chars().collect();
    v.shuffle(&mut rng);
    v.iter().collect::<String>()
}



// Copy of an alphabet rearranged to start with the key
pub fn keyed_alphabet(keyword: &str, alphabet: &str) -> String {
    let mut keyed_alpha = "".to_string();
    for k in keyword.chars() {
        let ks = &k.to_string();
        if !alphabet.contains(ks) {
            panic!("keyword must use symbols from the alphabet: {}",alphabet)
        }
        if keyed_alpha.contains(ks) {
            continue
        } else {
            keyed_alpha.push(k)
        }
    }

    for a in alphabet.chars() {
        if keyed_alpha.contains(&a.to_string()) {
            continue
        } else {
            keyed_alpha.push(a)
        }
    }
    keyed_alpha
}



pub fn validate_alphabet(alphabet: &str) -> bool {
    // No control characters, sorry
    for s in alphabet.chars() {
        if s.is_control() {
            return false
        }
    }
    // Check for combining characters maybe?
    // Characters must be unique
    alphabet.chars().count() == alphabet.chars().unique().count()
}



// confirm that they're not doing anything weird with unicode that could trip us up
#[test]
fn check_alphabets() {
    for s in CYRILLIC33.chars() {
        print!("{} ",s)
    }
    println!();

    for s in GREEK24.chars() {
        print!("{} ",s)
    }
    println!();
}
