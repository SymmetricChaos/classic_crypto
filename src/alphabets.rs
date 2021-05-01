use rand::{seq::SliceRandom, thread_rng};

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

/// The 95 printing ASCII characters
pub const ASCII95: &'static str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

/// As ASCII95 with the space removed
pub const ASCII94: &'static str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~"; 

/// The 24 letter modern Greek alphabet
pub const GREEK24: &'static str = "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ"; 

pub fn scramble_alphabet(alphabet: &str) -> String {
    let mut rng = thread_rng();
    let mut v: Vec<char> = alphabet.chars().collect();
    v.shuffle(&mut rng);
    v.iter().collect::<String>()
}