use std::{collections::HashMap, ops::Shr};
use itertools::Itertools;
use rand::{seq::index::sample, thread_rng};


pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}



fn egcd(a: i64, b: i64) -> (i64,i64,i64) {
    if a == 0 {
        (b,0,1)
    } else {
        let (g, y, x) = egcd(b%a, a);
        (g,x-(b/a)*y,y)
    }
}



pub fn mul_inv(num: usize, modulus: usize) -> Option<usize> {
    let (g, x, _) = egcd(num  as i64, modulus as i64);
    if g != 1 {
        None 
    } else {
        let t = x as usize;
        Some( t.rem_euclid(modulus) )
    }
}



pub fn log2(n: usize) -> usize {
    let mut ctr = 0;
    let mut n = n;
    while n != 0 {
        ctr += 1;
        n = n.shr(1);
    }
    ctr
}



pub fn rank_str(text: &str, alphabet: &str) -> Vec<usize> {
    let mut values = text.chars().map(|x| alphabet.chars().position(|c| x == c).unwrap()).collect::<Vec<usize>>();

    let len = values.len();
    let biggest = alphabet.len();

    let mut out = vec![0usize;len];

    for i in 0..len {
        let m = values.iter().min().unwrap();
        for (pos,v) in values.iter().enumerate() {
            if v == m {
                out[pos] = i;
                values[pos] = biggest;
                break
            }
        }
    }

    out
}



pub fn string_to_nums(text: &str, alphabet: &str) -> Vec<usize> {
    text.chars().map(|c| alphabet.chars().position(|x| x == c).unwrap() ).collect()
}



// Drop any symbols not used in the alphabet
pub fn strip_unused(text: &str, alphabet: &str) -> String {
    let mut out = String::new();
    for s in text.chars() {
        if alphabet.contains(s) {
            out.push(s)
        }
    }
    out
}



pub fn groups(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let ctr = [0,0,0,0,1].iter().cycle();
    for (c, n) in text.chars().zip(ctr) {
        out.push(c);
        if *n == 1 {
            out.push(' ')
        }
    }
    out
}



// Pad with random characters taken from the symbols provided
pub fn pad_end_with(text: &str, symbols: &str, n: usize) -> String {
    let mut out = text.to_string();
    let mut rng = thread_rng();
    let v = symbols.chars().collect_vec();
    for i in sample(&mut rng, n, v.len()).iter() {
        out.push(v[i])
    }
    out
}

pub fn pad_start_with(text: &str, symbols: &str, n: usize) -> String {
    let mut rng = thread_rng();
    let v = symbols.chars().collect_vec();
    let mut head = String::new();
    for i in sample(&mut rng, n, v.len()).iter() {
        head.push(v[i])
    }
    head.push_str(text);
    head
}

// Pad with symbols bootstrapped from the text itself. This padding doesn't stand out as much but could be confusing.
pub fn pad_end_bootstrap(text: &str, n: usize) -> String {
    let mut out = text.to_string();
    let mut rng = thread_rng();
    let v = text.chars().collect_vec();
    for i in sample(&mut rng, n, v.len()).iter() {
        out.push(v[i])
    }
    out
}

pub fn pad_start_bootstrap(text: &str, n: usize) -> String {
    let mut rng = thread_rng();
    let v = text.chars().collect_vec();
    let mut head = String::new();
    for i in sample(&mut rng, n, v.len()).iter() {
        head.push(v[i])
    }
    head.push_str(text);
    head
}


pub struct PrimeSieve {
    sieve: HashMap::<usize,Vec<usize>>,
    n: usize,
}

impl PrimeSieve {
    pub fn new() -> PrimeSieve {
        PrimeSieve{
            sieve: HashMap::<usize,Vec<usize>>::new(),
            n: 1usize}
    }
}

impl Iterator for PrimeSieve {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        loop {
            self.n += 1;
            if !self.sieve.contains_key(&self.n) {
                self.sieve.insert(self.n+self.n,vec![self.n]);
                return Some(self.n)
            } else {
                let factors = &self.sieve[&self.n].clone();
                for factor in factors {
                    if self.sieve.contains_key(&(factor+self.n)) {
                        self.sieve.get_mut(&(factor+self.n)).unwrap().push(*factor);
                    } else {
                        self.sieve.insert(factor+self.n,vec![*factor]);
                    }
                }
                self.sieve.remove(&self.n);
            }
        }
    }
}





#[test]
fn check_ranker() {
    assert_eq!(rank_str("ACDC","ABCDEFGHIJKLMNOPQRSTUVWXYZ"),vec![0,1,3,2]);
}


#[test]
fn test_groups() {
    assert_eq!(groups("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),"ABCDE FGHIJ KLMNO PQRST UVWXY Z");
}