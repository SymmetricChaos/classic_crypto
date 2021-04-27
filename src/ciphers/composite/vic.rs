use std::fmt;
use crate::modulus::Modulo;

/* fn string_to_sequence(text: &str) -> Vec<Modulo> {

} */

// The VIC Cipher is probably the strongest and certainly the most complex cipher known to have been used entirely by hand.
fn vic_block_generation(phrase: &str, date: Vec<usize>, pin: usize, keygroup: Vec<usize>) {

    let line_a: Vec<Modulo> = keygroup.iter().map(|x| Modulo::new(*x as u32, 10)).collect();
    let line_b: Vec<Modulo> = date[..5].iter().map(|x| Modulo::new(*x as u32, 10)).collect();
    let line_c: Vec<Modulo> = {
        let mut v = Vec::new();
        for (a,b) in line_a.iter().zip(line_b) {
            v.push(b-*a);
        }
        v
    };
    let line_d = &phrase[0..20];


}

pub struct VIC {
    phrase: String,
    date: Vec<usize>,
    pin: usize,
    keygroup: Vec<usize>,
}