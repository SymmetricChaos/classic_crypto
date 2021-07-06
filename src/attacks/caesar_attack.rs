// There are only 26 possibly keys for the Caesar cipher so trying them all is trivial. The challenge in automating the results in scoring the text.

use crate::{Cipher, ciphers::monoalphabetic::Caesar};
use crate::attacks::score_text::score_text_monogram;


// should return best solution and the key that produced it
pub fn caesar_attack(text: &str, alphabet: &str) -> (String,usize) {
    let alen = alphabet.chars().count();
    let mut best_decrypt = String::with_capacity(text.chars().count());
    let mut best_key = 0;
    let mut best_score = f64::INFINITY;
    for n in 0..alen {
        let decrypter = Caesar::new(n, alphabet);
        let candidate = decrypter.decrypt(text);
        let candidate_score = score_text_monogram(&candidate);
        if candidate_score < best_score {
            best_decrypt = candidate;
            best_key = n;
            best_score = candidate_score
        }
    }
    (best_decrypt,best_key)
}