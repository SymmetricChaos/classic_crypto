use std::collections::{HashMap,HashSet};

use itertools::Itertools;
use rand::{Rng,distributions::Uniform};
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;


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

pub struct Nomenclator<'a> {
    map: HashMap<String, Vec<String>>,
    map_inv: HashMap<String, String>,
    alphabet: &'a str, // alphabet used to construct the code groups
    code_width: usize, // number of symbols in each code group
}

// Needs to include ordinary code groups, nulls, and super nulls that remove the next code group
// use NUL for null and DEL for super null

impl Nomenclator {
    pub fn new(vocabulary: Vec<(&str, usize)>, code_alphabet: &str, code_wdith: &str, seed: u64) -> Nomenclator {

        let mut codes = CodeGroups::new(code_alphabet, code_alphabet, seed);

        let map = HashMap::with_capacity(vocabulary.len());
        let map_inv = HashMap::new();
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
            map.insert(word.to_string(), v);
        }
        Nomenclator{ map, map_inv, alphabet: code_alphabet, code_width }
    }
}

#[test]
fn test_code_iter() {
    let c = CodeGroups::new("ABCD",3, 0);
    for i in c.into_iter() {
        println!("{}",i)
    }
}