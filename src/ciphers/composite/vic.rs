use std::fmt;

use crate::auxiliary::rank_str;
use crate::alphabets::LATIN36;




/// The VIC Cipher is probably the strongest and certainly the most complex cipher known to have been used entirely by hand.
fn vic_block_generation(phrase: &str, date: Vec<u8>, pin: u8, keygroup: Vec<u8>) -> (Vec<u8>,Vec<u8>,Vec<u8>) {

    // The keygroup
    let line_a: Vec<u8> = keygroup.clone();
    println!("A: {}",vec_to_string(&line_a));
    
    // First five digits of the date
    let line_b: Vec<u8> = date[..5].iter().map(|x| *x).collect();
    println!("B: {}",vec_to_string(&line_b));

    //Subtract B from A
    let line_c: Vec<u8> = {
        let mut v = Vec::new();
        for (a,b) in line_a.iter().zip(line_b) {
            v.push((10+*a-b)%10);
        }
        v
    };
    println!("C: {}",vec_to_string(&line_c));

    // write down the phrase
    let line_d = &phrase[0..20];
    println!("D: {}  {}",&line_d[0..10],&line_d[10..20]);

    // rank the letters of each part of D using the VIC method
    let line_e1 = vic_rank(&phrase[0..10]);
    let line_e2 = vic_rank(&phrase[10..20]);
    println!("E: {}  {}",vec_to_string(&line_e1),vec_to_string(&line_e2));
    
    // Use chain addition to extend C out to ten terms, then the digits from 1 to 9, followed by 0
    let line_f = vic_chain_addition(line_c,10);
    let line_f1 = line_f[0..10].to_vec();
    let line_f2 = vec![1,2,3,4,5,6,7,8,9,0];
    println!("F: {}  {}",vec_to_string(&line_f1), vec_to_string(&line_f2));
    
    // Add E1 to F1
    let line_g = vic_vec_add(line_e1, line_f1);
    println!("G: {}",vec_to_string(&line_g));
    
    // Use E2 to encode G
    let line_h = vic_vec_encode(line_g, line_e2, line_f2);
    println!("H: {}",vec_to_string(&line_h));
    
    // Rank the digits of H
    let line_j = vic_rank_nums(&line_h);
    println!("J: {}",vec_to_string(&line_j));

    // produce the BLOCK by chain addition of H out to 60 terms
    let block = vic_chain_addition(line_h, 60);
    let line_k = block[10..20].to_vec();
    println!("K: {}",vec_to_string(&line_k));
    let line_l = block[20..30].to_vec();
    println!("L: {}",vec_to_string(&line_l));
    let line_m = block[30..40].to_vec();
    println!("M: {}",vec_to_string(&line_m));
    let line_n = block[40..50].to_vec();
    println!("N: {}",vec_to_string(&line_n));
    let line_p = block[50..60].to_vec();
    println!("P: {}",vec_to_string(&line_p));

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

    // The keys for Columnar Transposition, Diagonal Transposition, and Straddling Checkerboard
    let line_q = columns[..key_len1 as usize].to_vec();
    let line_r = columns[(key_len1 as usize)..((key_len1+key_len2) as usize)].to_vec();
    let line_s = vic_rank_nums(&line_p);
    println!("\nQ: {} (key 1)", vec_to_string(&line_q));
    println!("R: {} (key 2)", vec_to_string(&line_r));
    println!("S: {} (key 3)", vec_to_string(&line_s));

    (line_q, line_r, line_s)
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
    phrase: String,
    date: Vec<u8>,
    pin: u8,
    keygroup: Vec<u8>,
}

#[test]
fn test_key_derivation() {
    vic_block_generation("TWASTHENIGHTBEFORECH",vec![1,3,9,1,9], 6, vec![7,2,4,0,1]);
}