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
    map: HashMap<&'a str, &'a str>,
    map_inv: HashMap<&'a str, &'a str>,
    alphabet: &'a str, // alphabet used to construct the code groups
    code_width: usize, // number of symbols in each code group
    valid: Vec<&'a str>, // code groups that produce output
    null: Vec<&'a str>, // code groups that give no output
    super_null: Vec<&'a str> // code groups that give no output BUT also delete the next group
}

// Needs to include ordinary code groups, nulls, and super nulls that remove the next code group
// use NUL for null and DEL for super null

#[test]
fn test_code_iter() {
    let c = CodeGroups::new("ABCD",3, 0);
    for i in c.into_iter() {
        println!("{}",i)
    }
}