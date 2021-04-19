use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};
use crate::modulus::*;

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
    cmap: HashMap<char,Modulo>,
    umap: HashMap<Modulo,char>,
}

impl CipherAlphabet {
    pub fn new(alphabet: &str) -> CipherAlphabet {
        let alpha: Vec<char> = alphabet.chars().collect();
        let mut cmap: HashMap<char,Modulo> = HashMap::new();
        let mut umap: HashMap<Modulo,char> = HashMap::new();
        for (p,c) in alpha.iter().enumerate() {
            let m = (p as u32).to_modulo(alphabet.len() as u32);
            cmap.insert(*c, m);
            umap.insert(m, *c);
        }
        CipherAlphabet{ alpha, cmap, umap }
    }

    pub fn char_to_val(&self, c: char) -> Modulo {
        self.cmap[&c]
    }

    pub fn val_to_char(&self, n: Modulo) -> char {
        self.umap[&n]
    }
}

impl fmt::Display for CipherAlphabet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}",self.alpha)
    }
}
