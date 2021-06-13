use std::fmt;
use std::collections::HashMap;

use itertools::Itertools;

use super::{Rotor, M209_ROTORS, Cage};


// The M209 was a reciprocal cipher

fn char_to_usize(c: char) -> usize {
    (c as u8 as usize) - 65
}

fn usize_to_char(n: usize) -> char {
    (n + 65) as u8 as char
}

fn beaufort_encrypt(n: usize, k: usize, l: usize) -> usize {
    (l + k - n) % l
}

#[derive(Clone,Debug)]
pub struct M209 {
    wheels: [Rotor; 6],
    cage: Cage,
}

impl M209 {
    pub fn new(pins: [&str; 6], lugs: [(usize,usize);27]) -> M209 {
        // Use pins to set the wheels
        // Use lugs to set the cage
        let mut wheels = M209_ROTORS.clone();
        for (r, p) in wheels.iter_mut().zip(pins) {
            r.set_pins(p)
        }
        let cage = Cage::new(lugs.to_vec());
        M209{ wheels, cage }
    }

    pub fn set_wheels(&mut self, settings: &str) {
        for (r, c) in self.wheels.iter_mut().zip(settings.chars()) {
            r.set_display(c)
        }
    }

    // The stepping is perfectly regular, every wheel steps one position each letter
    // The encryption is done by the interaction of the lugs with the active pins
    pub fn step(&mut self) {
        for w in self.wheels.iter_mut() {
            w.step()
        }
    }

    pub fn step_n(&mut self, n: usize) {
        for _ in 0..n {
            for w in self.wheels.iter_mut() {
                w.step()
            }
        }

    }

    pub fn encrypt(&mut self, text: &str) -> String {
        let nums = text.chars().map(|x| char_to_usize(x)).collect_vec();
        let mut out = String::with_capacity(text.len());
        
        for n in nums {
            let mut sh = 0;
            for (lug_a, lug_b) in self.cage.bars.iter() {
                let mut a_effect = false;
                let mut b_effect = false;
                if lug_a == &0 {
                    // do nothing
                } else {
                    if self.wheels[lug_a-1].active_is_effective() {
                        a_effect = true;
                    }
                }
                if lug_b == &0 {
                    // do nothing
                } else {
                    if self.wheels[lug_b-1].active_is_effective() {
                        b_effect = true;
                    }
                }

                if a_effect || b_effect {
                    sh += 1
                }

            }
            let c = usize_to_char(beaufort_encrypt(n,sh,26));
            out.push(c);
            
            self.step();
            // first iterate over the bars of the cage 
            // for each lug on the bar check if that wheel has an effective pin in the active position
            // if either does sh += 1
            // then beaufort encrypt the n with the key sh
            // finally advance the wheels
        }
        out
    }

    // The M209 is reciprocal
    pub fn decrypt(&mut self, text: &str) -> String {
        self.encrypt(text)
    }

}


impl fmt::Display for M209 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "M209\n\nWheels\n".to_string();
        for w in self.wheels.iter() {
            s.push_str(&w.to_string());
            s.push('\n')
        }
        s.push('\n');
        s.push_str(&self.cage.to_string());
        write!(f, "{}",s)
    }
}

#[test]
fn test_m209_step() {
    let pins = ["ABDHIKMNSTVW",
                            "ADEGJKLORSUX",
                            "ABGHJLMNRSTUX",
                            "CEFHIMNPSTU",
                            "BDEFHIMNPS",
                            "ABDHKNOQ"];
    let lugs = [(3,6), (0,6), (1,6), (1,5), (4,5), (0,4), (0,4), (0,4), (0,4), (2,0), (2,0), (2,0), (2,0), (2,0), (2,0), (2,0), (2,0), (2,0), (2,0), (2,5), (2,5), (0,5), (0,5), (0,5), (0,5), (0,5), (0,5)];
    let mut m209 = M209::new(pins, lugs);
    for w in m209.wheels.iter() {
        println!("{}",w.get_active())
    }
    println!("{}",m209);
    println!("{}",m209.encrypt("AAAAAAAAAA"));
    m209.set_wheels("AAAAAA");
    println!("{}",m209.decrypt("UBDXZTLRZD"));
}