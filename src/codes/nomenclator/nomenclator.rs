use std::collections::{HashMap,HashSet};

use itertools::Itertools;
use rand::{Rng,distributions::Uniform};
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;

use crate::Code;


struct CodeGroups {
    alphabet: Vec<char>, // alphabet used to construct the code groups
    code_width: usize, // number of symbols in each code group
    alpha_len: usize,
    rng: Xoshiro256StarStar,
    used: HashSet<String>,
    maximum: usize,
}

impl CodeGroups {
    pub fn new(alphabet: &str, code_width: usize, seed: u64) -> CodeGroups {
        let alphabet = alphabet.chars().collect_vec();
        let alpha_len = alphabet.len();
        let rng = Xoshiro256StarStar::seed_from_u64(seed);
        let used = HashSet::new();
        let maximum = alpha_len.checked_pow(code_width as u32).unwrap();
        CodeGroups{ alphabet, code_width, alpha_len, rng, used, maximum }
    }
}

impl Iterator for CodeGroups {
    type Item = String;

    fn next(&mut self) -> Option::<Self::Item> {

        if self.used.len() == self.maximum {
            return None
        }

        let mut s = String::with_capacity(self.code_width);

        loop {
            let mut out_vec = Vec::with_capacity(self.code_width);

            for _ in 0..self.code_width {
                let x = self.rng.sample(Uniform::new(0,self.alpha_len));
                out_vec.push(self.alphabet[x])
            }

            s = out_vec.iter().collect::<String>();
            if !self.used.contains(&s) {
                self.used.insert(s.clone());
                break
            }
        }

        Some(s)
    }

}

pub struct Nomenclator {
    map: HashMap<String, Vec<String>>,
    map_inv: HashMap<String, String>,
    alphabet: String, // alphabet used to construct the code groups
    code_width: usize, // number of symbols in each code group
    vocabulary: Vec<String>,
}

// Needs to include ordinary code groups, nulls, and super nulls that remove the next code group
// use NUL for null and DEL for super null

impl Nomenclator {
    pub fn new(vocabulary: Vec<(&str, usize)>, code_alphabet: &str, code_width: usize, seed: u64) -> Nomenclator {

        let mut codes = CodeGroups::new(code_alphabet, code_width, seed);

        let mut map = HashMap::with_capacity(vocabulary.len());
        let mut map_inv = HashMap::new();
        let mut vocab = Vec::with_capacity(vocabulary.len());

        for (word, count) in vocabulary {
            let mut v = Vec::with_capacity(count);
            for _ in 0..count {
                if let Some(code) = codes.next() {
                    map_inv.insert(code.clone(),word.to_string());
                    v.push(code)
                } else {
                    panic!("Ran out of codes")
                }
            }
            vocab.push(word.to_string());
            map.insert(word.to_string(), v);
        }

        Nomenclator{ map, map_inv, alphabet: code_alphabet.to_string(), code_width, vocabulary: vocab }
    }

}

impl crate::Code for Nomenclator {

    fn encode(&self, text: &str) -> String {
        let mut rng = Xoshiro256StarStar::from_entropy();
        let mut out = String::new();
        let mut buffer = String::new();

        for t in text.chars() {
            buffer.push(t);
            if let Some(code) = self.map.get(&buffer) {
                //choose randomly
                let r = rng.gen_range(0..code.len());
                out.push_str(&code[r]);
                buffer = String::new();
            }
        }

        out
    }

    fn decode(&self, text: &str) -> String {
        let mut out = String::new();
        let groups = text.chars().chunks(self.code_width).into_iter().map(|x| x.collect::<String>()).collect_vec();
        for g in groups {
            out.push_str( &self.map_inv[&g] )
        }
        out
    }

    fn char_map(&self) -> String {
        let mut out = String::new();
        for v in self.vocabulary.iter() {
            let s = &format!("{}  {:?}\n", v, self.map[v]);
            out.push_str(s)
        }
        out
    }

}

#[test]
fn test_code_iter() {
    let c = CodeGroups::new("ABCD",3, 0);
    for i in c.into_iter() {
        println!("{}",i)
    }
}

#[test]
fn test_nomenclator() {
    let vocab = vec![
        ("AA",3),("AB",3),("AC",3),("AD",3),
        ("BA",3),("BB",3),("BC",3),("BD",3),
        ("CA",3),("CB",3),("CC",3),("CD",3),
        ("DA",3),("DB",3),("DC",3),("DD",3),
    ];
    let nom = Nomenclator::new(vocab, "0123456789", 3, 837);

    let plaintext = "ABDCDD";
    let encoded = nom.encode(plaintext);
    let decoded = nom.decode(&encoded);

    println!("{}",nom.char_map());
    println!("{}",encoded);
    println!("{}",decoded);
}