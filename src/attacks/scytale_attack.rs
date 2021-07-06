// There are roughly n/2 useable keys for a scytale where n is the number of characters in the plaintext though in practice the key is likely small

use crate::{Cipher, ciphers::transposition::Scytale};
use crate::attacks::score_text::score_text_bigram;


// should return best solution and the key that produced it
pub fn scytale_attack(text: &str) -> (String,usize) {
    let tlen = text.chars().count();
    let mut best_decrypt = String::with_capacity(text.chars().count());
    let mut best_key = 0;
    let mut best_score = f64::INFINITY;
    for n in 2..tlen/2 {
        let decrypter = Scytale::new(n);
        let candidate = decrypter.decrypt(text);
        let candidate_score = score_text_bigram(&candidate);
        if candidate_score < best_score {
            best_decrypt = candidate;
            best_key = n;
            best_score = candidate_score
        }
    }
    (best_decrypt,best_key)
}