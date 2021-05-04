use std::fmt;
use crate::modulus::Modulo;
use crate::auxiliary::rank_str;
use crate::alphabets::LATIN26;


/// The VIC Cipher is probably the strongest and certainly the most complex cipher known to have been used entirely by hand.
fn vic_block_generation(phrase: &str, date: Vec<usize>, pin: usize, keygroup: Vec<usize>) {

    let line_a: Vec<Modulo> = keygroup.iter().map(|x| Modulo::new(*x as u32, 10)).collect();
    println!("A: {:?}",line_a);
    let line_b: Vec<Modulo> = date[..5].iter().map(|x| Modulo::new(*x as u32, 10)).collect();
    println!("B: {:?}",line_b);
    let line_c: Vec<Modulo> = {
        let mut v = Vec::new();
        for (a,b) in line_a.iter().zip(line_b) {
            v.push(b-*a);
        }
        v
    };
    println!("C: {:?}",line_c);
    let line_d = &phrase[0..20];
    println!("D: {:?}",line_d);
    let line_e1 = rank_str(&phrase[0..10],LATIN26);
    let line_e2 = rank_str(&phrase[10..20],LATIN26);
    


}

pub struct VIC {
    phrase: String,
    date: Vec<usize>,
    pin: usize,
    keygroup: Vec<usize>,
}