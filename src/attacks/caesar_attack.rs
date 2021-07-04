// There are only 26 possibly keys for the Caesar cipher so trying them all is trivial. The challenge in automating the results in scoring the text.

use crate::{Cipher, alphabets::LATIN26, ciphers::monoalphabetic::Caesar};
use crate::attacks::score_text::score_text_monogram;

#[test]
fn example() {
    let mut texts = Vec::new();
    for n in 0..26 {
        let caesar = Caesar::new(n, LATIN26);
        texts.push(caesar.encrypt("THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG"))
    }
    for t in texts.iter() {
        println!("{}",score_text_monogram(t))
    }
}