use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::modulus::*;

lazy_static! {
    pub static ref ALPHA26: ModularAlphabet    = ModularAlphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    pub static ref ALPHA_NUM: ModularAlphabet  = ModularAlphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    pub static ref ALPHA25_CK: ModularAlphabet = ModularAlphabet::new("ABDEFGHIJKLMNOPQRSTUVWXYZ");
    pub static ref ALPHA25_IJ: ModularAlphabet = ModularAlphabet::new("ABCDEFGHIKLMNOPQRSTUVWXYZ");
}

#[derive(Clone)]
pub struct ModularAlphabet {
    cmap: HashMap<char,Modulo>,
    vmap: HashMap<Modulo,char>,
    length: usize,
}

impl ModularAlphabet {
    pub fn new(alphabet: &str) -> ModularAlphabet {
        let alpha: Vec<char> = alphabet.chars().collect();
        let length = alpha.len();
        let mut cmap: HashMap<char,Modulo> = HashMap::new();
        let mut vmap: HashMap<Modulo,char> = HashMap::new();
        for (p,c) in alpha.iter().enumerate() {
            let m = (p as u32).to_modulo(alphabet.len() as u32);
            cmap.insert(*c, m);
            vmap.insert(m, *c);
        }
        ModularAlphabet{ cmap, vmap, length }
    }

    pub fn char_to_val(&self, c: char) -> Option<&Modulo> {
        self.cmap.get(&c)
    }

    pub fn val_to_char(&self, n: Modulo) -> Option<&char> {
        self.vmap.get(&n)
    }

    pub fn len(&self) -> usize {
        self.length
    }
}