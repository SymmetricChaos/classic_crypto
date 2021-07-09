use std::collections::HashMap;
use crate::attacks::score_text::score_text_monogram;

// A Vigenere cipher is ultimately just a sequence of Caesar ciphers and those are easy to break.
// To break a Vigenere or similar polyalphabetic cipher with a cyclic key Kasiski examination can be used

pub fn kasiski(text: &str) -> Vec<usize> {
    // scan through trigrams for repetitions and determine distance between them
    let mut chs = text.chars();
    let mut buffer = String::with_capacity(3);
    buffer.push(chs.next().unwrap());
    buffer.push(chs.next().unwrap());
    buffer.push(chs.next().unwrap());

    let mut pos = 0;
    let mut diffs: Vec<usize> = Vec::new();

    let mut tri: HashMap<String,Vec<usize>> = HashMap::new();
    tri.insert(buffer.clone(), vec![0]);

    while let Some(c) = chs.next() {
        buffer.remove(0);
        buffer.push(c);
        pos += 1;
        let mut entry = tri.get_mut(&buffer.clone());
        if let Some(v) = entry {
            v.push(pos);
        } else {
            tri.insert(buffer.clone(), vec![pos]);
        }
        
    }

    for k in tri.keys() {
        println!("{} {:?}",k,tri[k])
    }

    let mut key_lens = Vec::new();
    for d in diffs.iter() {

    }


    key_lens
}

/* // should return best solution and the key that produced it
pub fn vigenere_attack(text: &str) -> (String,usize) {
    let key_lens = kasiski(text);
}
 */

#[cfg(test)]
mod attack_tests {
    use crate::attacks::kasiski;
    use crate::ciphers::{
        polyalphabetic::Vigenere};
    use crate::Cipher;
    use crate::alphabets::LATIN26;

    #[test]
    fn test_kasiski() {
        let cipher = Vigenere::new("SECRET", LATIN26);
        let plaintext = "ATTACKINGATTACKINGATTACKINGATTACKINGATTACKING";
        let ciphertext = cipher.encrypt(plaintext);
        println!("{:?}",ciphertext);
        kasiski(&ciphertext);

    }

}