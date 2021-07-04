use std::fmt;
use std::{fs::File, io::{Error, Read, Write}};

use crate::auxiliary::rank_str;
use crate::alphabets::LATIN36;
use crate::ciphers::{transposition::Columnar,VicCheckerboard};
use crate::Cipher;



/// The VIC Cipher is probably the strongest and certainly the most complex cipher known to have been used entirely by hand.
fn vic_block_generation(keygroup: Vec<u8>, date: Vec<u8>, pin: u8, phrase: &str) -> (Vec<u8>,Vec<u8>,Vec<u8>,String) {

    let mut derivation = String::new();

    // The keygroup
    let line_a: Vec<u8> = keygroup.clone();
    derivation.push_str(&format!("A: {}\n",vec_to_string(&line_a)));
    
    // First five digits of the date
    let line_b: Vec<u8> = date[..5].iter().map(|x| *x).collect();
    derivation.push_str(&format!("B: {}\n",vec_to_string(&line_b)));

    //Subtract B from A
    let line_c: Vec<u8> = {
        let mut v = Vec::new();
        for (a,b) in line_a.iter().zip(line_b) {
            v.push((10+*a-b)%10);
        }
        v
    };
    derivation.push_str(&format!("C: {}\n",vec_to_string(&line_c)));

    // write down the phrase
    derivation.push_str(&format!("D: {}  {}\n",&phrase[0..10],&phrase[10..20]));

    // rank the letters of each part of D using the VIC method
    let line_e1 = vic_rank(&phrase[0..10]);
    let line_e2 = vic_rank(&phrase[10..20]);
    derivation.push_str(&format!("E: {}  {}\n",vec_to_string(&line_e1),vec_to_string(&line_e2)));
    
    // Use chain addition to extend C out to ten terms, then the digits from 1 to 9, followed by 0
    let line_f = vic_chain_addition(line_c,10);
    let line_f1 = line_f[0..10].to_vec();
    let line_f2 = vec![1,2,3,4,5,6,7,8,9,0];
    derivation.push_str(&format!("F: {}  {}\n",vec_to_string(&line_f1), vec_to_string(&line_f2)));
    
    // Add E1 to F1
    let line_g = vic_vec_add(line_e1, line_f1);
    derivation.push_str(&format!("G: {}\n",vec_to_string(&line_g)));
    
    // Use E2 to encode G
    let line_h = vic_vec_encode(line_g, line_e2, line_f2);
    derivation.push_str(&format!("H: {}\n",vec_to_string(&line_h)));
    
    // Rank the digits of H
    let line_j = vic_rank_nums(&line_h);
    derivation.push_str(&format!("J: {}\n",vec_to_string(&line_j)));

    // produce the BLOCK by chain addition of H out to 60 terms
    let block = vic_chain_addition(line_h, 60);
    let line_k = block[10..20].to_vec();
    derivation.push_str(&format!("K: {}\n",vec_to_string(&line_k)));
    let line_l = block[20..30].to_vec();
    derivation.push_str(&format!("L: {}\n",vec_to_string(&line_l)));
    let line_m = block[30..40].to_vec();
    derivation.push_str(&format!("M: {}\n",vec_to_string(&line_m)));
    let line_n = block[40..50].to_vec();
    derivation.push_str(&format!("N: {}\n",vec_to_string(&line_n)));
    let line_p = block[50..60].to_vec();
    derivation.push_str(&format!("P: {}\n",vec_to_string(&line_p)));

    // Get the last to unequal digit of P and add them to the pin to get the key lengths
    let last_pair = last_unequal(&line_p);
    let key_len1 = pin + last_pair.0;
    let key_len2 = pin + last_pair.1;

    // Read the BLOCK off by columns in the order given by J
    let columns = {
        let digits = [1,2,3,4,5,6,7,8,9,0];
        let col_pos = digits.iter().map(|x| line_j.iter().position(|p| p == x).unwrap());
        let mut out = Vec::with_capacity(50);
        for c in col_pos {
            out.push(line_k[c]);
            out.push(line_l[c]);
            out.push(line_m[c]);
            out.push(line_n[c]);
            out.push(line_p[c]);
        }
        out
    };

    // The keys for IncompleteColumnar Transposition, Diagonal Transposition, and Straddling Checkerboard
    let line_q = columns[..key_len1 as usize].to_vec();
    let line_r = columns[(key_len1 as usize)..((key_len1+key_len2) as usize)].to_vec();
    let line_s = vic_rank_nums(&line_p);
    derivation.push_str(&format!("Q: {} (key 1)\n", vec_to_string(&line_q)));
    derivation.push_str(&format!("R: {} (key 2)\n", vec_to_string(&line_r)));
    derivation.push_str(&format!("S: {} (key 3)\n", vec_to_string(&line_s)));

    (line_q, line_r, line_s, derivation)
}

fn vec_to_string(v: &Vec<u8>) -> String {
    v.iter().map(|n| format!("{}",n) ).collect::<String>()
}

fn vic_vec_add(v1: Vec<u8>, v2: Vec<u8>) -> Vec<u8> {
    let mut out = Vec::with_capacity(v1.len());
    for (a,b) in v1.iter().zip(v2) {
        out.push( (*a + b) % 10 )
    }
    out
}

// this can be simplified since f is fixed in practice
fn vic_vec_encode(v1: Vec<u8>, v2: Vec<u8>, f: Vec<u8>) -> Vec<u8> {
    let mut out = Vec::with_capacity(v1.len());
    for n in v1 {
        let x = f.iter().position(|a| *a == n).unwrap();
        out.push(v2[x])
    }
    out
}

// Rank and then shift by one to reflect that the VIC cipher treated 0 as the highest position
fn vic_rank(text: &str) -> Vec<u8> {
    rank_str(text, LATIN36).iter().map(|x| (*x as u8 + 1) % 10).collect()
}

fn vic_rank_nums(v: &Vec<u8>) -> Vec<u8> {
    let arr = [1u8,2,3,4,5,6,7,8,9,0];
    let mut values = v.iter().map(|x| arr.iter().position(|c| x == c).unwrap() as u8).collect::<Vec<u8>>();

    let len = values.len();
    let biggest = 10;

    let mut out = vec![0u8;len];

    for i in 0..len {
        let m = values.iter().min().unwrap();
        for (pos,v) in values.iter().enumerate() {
            if v == m {
                out[pos] = ((i+1)%10) as u8;
                values[pos] = biggest;
                break
            }
        }
    }

    out
}

// final version should do this in place but for checking the method this is easier to see
fn vic_chain_addition(seed: Vec<u8>, n: u8) -> Vec<u8> {
    let mut out = seed.clone();
    let mut ctr = 0;
    while out.len() < n.into() {
        let x = (out[ctr] + out[ctr+1]) % 10;
        out.push(x);
        ctr += 1
    }
    out
}

fn last_unequal(row: &Vec<u8>) -> (u8,u8) {
    let mut nums = row.iter().rev();
    let mut b = nums.next().unwrap();
    let mut a = nums.next().unwrap();
    while a == b {
        b = a;
        a = nums.next().unwrap();
    }
    return (*a,*b)
}


pub struct VIC {
    key1: Vec<u8>,
    key2: Vec<u8>,
    key3: Vec<u8>,
    derivation: String,
}

impl VIC {
    pub fn new(keygroup: Vec<u8>, date: Vec<u8>, pin: u8, phrase: &str) -> VIC {
        let (key1, key2, key3, derivation) = vic_block_generation(keygroup,date,pin,phrase);

        VIC{ key1, key2, key3, derivation }
    }

    pub fn derivation(&self) -> String {
        self.derivation.clone()
    }


    // https://derekbruff.org/blogs/fywscrypto/historical-crypto/handwritten-russian-cipher-stumps-fbi/
    // https://technears.wordpress.com/current-issue/technears-1-2/workflow-analysis/mr-gaskells-spy-vs-spy-workflow-analysis/
    // Need to insert and extract the keygroup

    fn encrypt(&self, text: &str) -> String {

        // Encrypt the text with the Straddling Checkerboard
        let checkerboard = VicCheckerboard::new(self.key3.clone());
        let mut ctext = checkerboard.encrypt(text);

        let columnar_key1 = vec_to_string(&self.key1);
        let columnar1 = Columnar::new(&columnar_key1,"1234567890");
        ctext = columnar1.encrypt(&ctext);

        // Supposedly this is a "diagonal transposition" but I can't find what they means yet
        let columnar_key2 = vec_to_string(&self.key2);
        let columnar2 = Columnar::new(&columnar_key2,"1234567890");
        ctext = columnar2.encrypt(&ctext);
        
        ctext
    }

    fn decrypt(&self, text: &str) -> String {

        let columnar_key2 = vec_to_string(&self.key2);
        let columnar2 = Columnar::new(&columnar_key2,"1234567890");
        let mut ptext = columnar2.decrypt(text);

        let columnar_key1 = vec_to_string(&self.key1);
        let columnar1 = Columnar::new(&columnar_key1,"1234567890");
        ptext = columnar1.decrypt(&ptext);

        let checkerboard = VicCheckerboard::new(self.key3.clone());
        ptext = checkerboard.decrypt(&ptext);

        ptext
    }
}



#[test]
fn test_key_derivation() {
    let v = VIC::new(vec![7,2,4,0,1], vec![1,3,9,1,9,5,9], 6, "TWASTHENIGHTBEFORECH" );
    println!("{}",v.derivation())
}

#[test]
fn test_vic() {
    let v = VIC::new(vec![7,2,4,0,1], vec![1,3,9,1,9,5,9], 6, "TWASTHENIGHTBEFORECH" );
    let plaintext = "ATTACKATDAWN";
    let ciphertext = v.encrypt(plaintext);
    let decrypted = v.decrypt(&ciphertext);

    println!("{}",ciphertext);
    assert_eq!(decrypted,plaintext)
}

#[test]
fn test_vic_long() {
    let v = VIC::new(vec![7,2,4,0,1], vec![1,3,9,1,9,5,9], 6, "TWASTHENIGHTBEFORECH" );
    let plaintext = "FORORGANIZATIONOFCOVERWEGAVEINSTRUCTIONSTOTRANSMITTOYOUTHREETHOUSANDINLOCALCURRENCY.CONSULTWITHUSPRIORTOINVESTINGITINANYKINDOFBUSINESSADVISINGTHECHARACTEROFTHISBUSINESS";
    let ciphertext = v.encrypt(plaintext);
    let decrypted = v.decrypt(&ciphertext);

    println!("{}",ciphertext);

    assert_eq!(decrypted,plaintext)
}