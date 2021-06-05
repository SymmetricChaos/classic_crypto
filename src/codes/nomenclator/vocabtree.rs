use std::{collections::{HashMap,HashSet}, fmt, str::Chars};

use itertools::Itertools;
use radix_trie::{Trie, TrieCommon};


#[derive(Clone,Debug)]
pub struct VocabNode {
    symbol: char,
    children: HashMap<char,VocabNode>,
}

impl VocabNode {

    pub fn new(symbol: char) -> VocabNode {
        VocabNode{ symbol, children: HashMap::new() }
    }

    pub fn insert_char(&mut self, symbol: char) {
        self.children.insert(symbol, VocabNode::new(symbol));
    }

    pub fn insert_chars(&mut self, symbols: &mut Chars) {
        if let Some(symbol) = symbols.next() {
            if !self.children.contains_key(&symbol) {
                self.insert_char(symbol);
                self.children.get_mut(&symbol).unwrap().insert_chars(symbols)
            } else {
                self.children.get_mut(&symbol).unwrap().insert_chars(symbols)
            }
        } else {
            self.insert_char('\0');
        }
    }

    pub fn read_out(&self, out: Vec<char>) {
        for (key, vals) in self.children.iter() {
            if key == &'\0' {
                println!("{}",out.iter().collect::<String>());
            } else {
                let mut t = out.clone();
                t.push(*key);
                vals.read_out(t)
            }
        }

    }

}

impl fmt::Display for VocabNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.symbol == '\0' {
            let mut s = String::new();
            for c in self.children.iter() {
                s.push_str(&format!("[{}]",c.1))
            }
            write!(f, "{}", s)
        } else {
            let mut s = format!("{}: ",self.symbol);
            for c in self.children.iter() {
                s.push_str(&format!("[{}]",c.1))
            }
            write!(f, "{}", s)
        }
    }
}


#[test]
fn test_trie() {
    let vocab = ["T","TH","THE","THAT","TEN"];
    let mut t = Trie::new();
    for i in 0..5 {

        t.insert(i, vocab[i]);
    }
    println!("{:?}",t);
    for x in t.values() {
        println!("{:?}",x)
    }
    println!("{:?}",t.prefix());
}


#[test]
fn test_custom_tree() {
    let mut t= VocabNode::new('\0');
    let word_list = ["THE","THERE","THAT","THESE","TANK"];
    for word in word_list {
        let mut symbols = word.chars();
        t.insert_chars(&mut symbols);

    }
    println!("{}",t);
    t.read_out(Vec::new());
}