use lazy_static::lazy_static;
use std::{collections::HashMap};

lazy_static! {
    pub static ref ALPHA: HashMap<u8,u8> = {
        let mut m = HashMap::new();
        for n in 0..26 {
            m.insert(n, n);
            m.insert(n+65, n);
            m.insert(n+97, n);
        }
        m
    };
}

pub struct CipherAlphabet {
    alpha: Vec<char>,
    cmap: HashMap<char,usize>,
    umap: HashMap<usize,char>,
}

impl CipherAlphabet {
    pub fn new(alphabet: &str) -> CipherAlphabet {
        let alpha: Vec<char> = alphabet.chars().collect();
        let mut cmap: HashMap<char,usize> = HashMap::new();
        let mut umap: HashMap<usize,char> = HashMap::new();
        for (p,c) in alpha.iter().enumerate() {
            cmap.insert(*c, p);
            umap.insert(p, *c);
        }
        CipherAlphabet{ alpha, cmap, umap }
    }

    pub fn char_to_usize(&self, c: char) -> usize {
        self.cmap[&c]
    }

    pub fn usize_to_char(&self, n: usize) -> char {
        self.umap[&n]
    }
}