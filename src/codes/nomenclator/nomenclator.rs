use std::collections::{HashMap,HashSet};

use itertools::Itertools;
use rand::{Rng,distributions::Uniform};
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;

use crate::Code;
use super::vocabtree::VocabNode;

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
    null_rate: f64,
    vocab_tree: VocabNode,
}

// Needs to include ordinary code groups, nulls, and super nulls that remove the next code group
// use NUL for null and DEL for super null

impl Nomenclator {
    pub fn new(vocabulary: Vec<(String, usize)>, code_alphabet: &str, code_width: usize, seed: u64, null_rate: f64) -> Nomenclator {

        let mut codes = CodeGroups::new(code_alphabet, code_width, seed);

        let mut map = HashMap::with_capacity(vocabulary.len());
        let mut map_inv = HashMap::new();
        let mut vocab = Vec::with_capacity(vocabulary.len());
        let mut vocab_tree = VocabNode::new('\0');

        for (word, count) in vocabulary {
            let mut v = Vec::with_capacity(count);
            for _ in 0..count {
                if let Some(code) = codes.next() {
                    map_inv.insert(code.clone(),word.to_string());
                    v.push(code)
                } else {
                    panic!("Ran out of code groups")
                }
            }
            vocab.push(word.to_string());
            vocab_tree.insert_chars(&mut word.chars());
            map.insert(word.to_string(), v);
        }

        Nomenclator{ map, map_inv, alphabet: code_alphabet.to_string(), code_width, vocabulary: vocab, null_rate, vocab_tree }
    }

    pub fn random_null(&self) -> String {
        let mut rng = Xoshiro256StarStar::from_entropy();

        loop {
            let mut out_vec = Vec::with_capacity(self.code_width);
            for _ in 0..self.code_width {
                let x = rng.sample(Uniform::new(0,self.alphabet.len()));
                out_vec.push(self.alphabet.chars().nth(x).unwrap())
            }
            let s = out_vec.iter().collect::<String>();
            if !self.map_inv.contains_key(&s) {
                return s
            }
        }


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
                if rng.gen_bool(self.null_rate) {
                    out.push_str(&self.random_null())
                }
                let r = rng.gen_range(0..code.len());
                out.push_str(&code[r]);
                buffer = String::new();
            }
        }
        if buffer != "".to_string() {
            panic!("unknown string `{}` found",buffer)
        }

        out
    }

    fn decode(&self, text: &str) -> String {
        let mut out = String::new();
        let groups = text.chars().chunks(self.code_width).into_iter().map(|x| x.collect::<String>()).collect_vec();
        for g in groups {
            out.push_str( &self.map_inv.get(&g).unwrap_or(&"".to_string()) )
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

// To allow vocabulary words that include prefixes of each other (ie "THE" and "THERE")  a trie data structure is needed and can be traversed until it is unable to continue
#[test]
fn test_nomenclator() {
    use crate::alphabets::LATIN26;
    use itertools::Itertools;
    let di_map = LATIN26.chars().cartesian_product(LATIN26.chars());
    let digraphs = di_map.map(|(a, b)| (format!("{}{}",a,b),3)).collect_vec();

    let nom = Nomenclator::new(digraphs, LATIN26, 3, 837, 0.2);

    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGX";
    let encoded = nom.encode(plaintext);
    let decoded = nom.decode(&encoded);

    //println!("{}",nom.char_map());
    println!("{}",encoded);
    println!("{}",decoded);
    assert_eq!(decoded,plaintext)
}