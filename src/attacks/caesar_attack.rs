// There are only 26 possibly keys for the Caesar cipher so trying them all is trivial. The challenge in automating the results in scoring the text.

use crate::{Cipher, alphabets::LATIN26, ciphers::monoalphabetic::Caesar};
use crate::attacks::score_text::score_text_monogram;


// should return best solution and the key that produced it
pub fn caesar_attack(text: &str, alphabet: &str) -> (String,usize) {
    let alen = alphabet.chars().count();
    for n in 0..alen {
        let decrypter = Caesar::new(n, alphabet);
        decrypter.decrypt(text)
    }
}
