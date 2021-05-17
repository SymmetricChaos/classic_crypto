use std::fmt;

use crate::auxiliary::rank_str;
use crate::alphabets::LATIN36;


/// The VIC Cipher is probably the strongest and certainly the most complex cipher known to have been used entirely by hand.
fn vic_block_generation(phrase: &str, date: Vec<u8>, pin: u8, keygroup: Vec<u8>) {

    let line_a: Vec<u8> = keygroup.clone();
    println!("A: {:?}",line_a);
    let line_b: Vec<u8> = date[..5].iter().map(|x| *x).collect();
    println!("B: {:?}",line_b);
    let line_c: Vec<u8> = {
        let mut v = Vec::new();
        for (a,b) in line_a.iter().zip(line_b) {
            v.push((10+*a-b)%10);
        }
        v
    };
    println!("C: {:?}",line_c);
    let line_d = &phrase[0..20];
    println!("D: {:?}",line_d);
    let line_e1 = vic_rank(&phrase[0..10]);
    let line_e2 = vic_rank(&phrase[10..20]);
    println!("E: {:?}  {:?}",line_e1,line_e2);
    let line_f = vic_chain_addition(line_c,10);
    let line_f1 = line_f[0..10].to_vec();
    let line_f2 = vec![1,2,3,4,5,6,7,8,9,0];
    println!("F: {:?}  {:?}",line_f1, line_f2);
    let line_g = vic_vec_add(line_e1, line_f1);
    println!("G: {:?}",line_g);
    let line_h = vic_vec_encode(line_g, line_e2, line_f2);
    println!("H: {:?}",line_h);
    let line_j = vic_rank(&vec_to_string(&line_h));
    println!("J: {:?}",line_j);
    let block = vic_chain_addition(line_h, 60);
    let line_k = block[10..20].to_vec();
    println!("K: {:?}",line_k);
    let line_l = block[20..30].to_vec();
    println!("L: {:?}",line_l);
    let line_m = block[30..40].to_vec();
    println!("M: {:?}",line_m);
    let line_n = block[40..50].to_vec();
    println!("N: {:?}",line_n);
    let line_p = block[50..60].to_vec();
    println!("P: {:?}",line_p);
    let last_pair = last_unequal(&line_p);
    let key_len1 = pin + last_pair.0;
    let key_len2 = pin + last_pair.1;
    println!("key lengths {}, {}", key_len1, key_len2);

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

    let line_q = &columns[..key_len1 as usize];
    let line_r = &columns[(key_len1 as usize)..((key_len1+key_len2) as usize)];
    println!("Q: {:?} (key 1)", line_q);
    println!("R: {:?} (key 2)", line_r);
    let line_s = vic_rank(&vec_to_string(&line_p));
    println!("S: {:?} <- this is wrong", line_s);
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
    vic_block_generation("TWASTHENIGHTBEFORECH",vec![1,3,9,1,9], 6, vec![7,2,4,0,1])
}