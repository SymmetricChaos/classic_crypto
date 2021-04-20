use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};
use crate::modulus::*;

lazy_static! {
    pub static ref ALPHA26: CipherAlphabet = CipherAlphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ");

    pub static ref ALPHA_NUM: CipherAlphabet = CipherAlphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");

}

#[derive(Clone)]
pub struct CipherAlphabet {
    alpha: Vec<char>,
    cmap: HashMap<char,Modulo>,
    vmap: HashMap<Modulo,char>,
}

impl CipherAlphabet {
    pub fn new(alphabet: &str) -> CipherAlphabet {
        let alpha: Vec<char> = alphabet.chars().collect();
        let mut cmap: HashMap<char,Modulo> = HashMap::new();
        let mut vmap: HashMap<Modulo,char> = HashMap::new();
        for (p,c) in alpha.iter().enumerate() {
            let m = (p as u32).to_modulo(alphabet.len() as u32);
            cmap.insert(*c, m);
            vmap.insert(m, *c);
        }
        CipherAlphabet{ alpha, cmap, vmap }
    }

    pub fn char_to_val(&self, c: char) -> Option<Modulo> {
        if self.cmap.contains_key(&c) {
            Some(self.cmap[&c])
        } else {
            None
        }
    }

    pub fn val_to_char(&self, n: Modulo) -> char {
        self.vmap[&n]
    }

    pub fn len(&self) -> u32 {
        self.alpha.len() as u32
    }
}

impl fmt::Display for CipherAlphabet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}",self.alpha)
    }
}
