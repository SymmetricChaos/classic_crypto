use std::collections::{HashMap,HashSet};
//use itertools::Itertools;

//use crate::attacks::score_text::score_text_monogram;
use crate::auxiliary::{factors, pairwise_diffs};

// A Vigenere cipher is ultimately just a sequence of Caesar ciphers and those are easy to break.
// To break a Vigenere or similar polyalphabetic cipher with a cyclic key Kasiski examination can be used

pub fn kasiski(text: &str) -> HashSet<usize> {
    // scan through trigrams for repetitions and determine distance between them
    let mut chs = text.chars();
    let mut buffer = String::with_capacity(3);
    buffer.push(chs.next().unwrap());
    buffer.push(chs.next().unwrap());
    buffer.push(chs.next().unwrap());

    let mut pos = 0;

    let mut tri: HashMap<String,Vec<usize>> = HashMap::new();
    tri.insert(buffer.clone(), vec![0]);

    while let Some(c) = chs.next() {
        buffer.remove(0);
        buffer.push(c);
        pos += 1;
        let entry = tri.get_mut(&buffer.clone());
        if let Some(v) = entry {
            v.push(pos);
        } else {
            tri.insert(buffer.clone(), vec![pos]);
        }
        
    }


    let mut diffs = HashSet::new();
    for (_, v) in tri {
        for pd in pairwise_diffs(v) {
            diffs.insert(pd);
        }
    }

    let mut key_lens = HashSet::new();
    for d in diffs {
        for f in factors(d) {
            key_lens.insert(f);
        }
    }

    key_lens
}

// should return best solution and the key that produced it
/* pub fn vigenere_attack(text: &str) -> (String,usize) {
    let key_lens = kasiski(text);
    for kl in key_lens {

    }

    (String::new, 0usize)
} */

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
        println!("{:?}", kasiski(&ciphertext));
    }

}