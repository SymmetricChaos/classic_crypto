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

/// The 24 letter modern Greek alphabet
pub const GREEK24: &'static str = "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ"; 

/// The Russian alphabet
pub const CYRILLIC33: &'static str = "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ"; 

pub fn scramble_alphabet(alphabet: &str) -> String {
    let mut rng = thread_rng();
    let mut v: Vec<char> = alphabet.chars().collect();
    v.shuffle(&mut rng);
    v.iter().collect::<String>()
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
