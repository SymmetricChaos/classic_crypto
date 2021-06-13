use itertools::Itertools;
use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};

#[derive(Clone,Debug)]
pub struct Cage {
    pub bars: Vec<(usize,usize)>
}


impl Cage {
    pub fn new(bars: Vec<(usize,usize)>) -> Cage {
        if bars.len() != 27 {
            panic!("there must be exactly 27 bars")
        }
        for b in bars.iter() {
            if b.0 > 6 {
                panic!("bar settings must 0, 1, 2, 3, 4, 5, or 6")
            }
            if b.1 > 6 {
                panic!("bar settings must 0, 1, 2, 3, 4, 5, or 6")
            }
        }
        Cage{ bars }
    }

}


impl fmt::Display for Cage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "Cage\n".to_string();
        for b in self.bars.chunks(3).collect_vec() {
            for lug in b {
                let entry = format!("{}-{}  ",lug.0,lug.1);
                s.push_str(&entry)
            }
            s.push('\n')
        }
        write!(f, "{}", s)
    }
}