use std::fmt;
use std::collections::VecDeque;
use std::iter::FromIterator;

use itertools::Itertools;


pub struct Chaocipher {
    alpha_l: VecDeque<char>,
    alpha_r: VecDeque<char>,
}

impl Chaocipher {
    pub fn new(alphabet_left: &str, alphabet_right: &str) -> Chaocipher {
        let alpha_l = VecDeque::from_iter(alphabet_left.chars());
        let alpha_r = VecDeque::from_iter(alphabet_right.chars());
        Chaocipher{ alpha_l, alpha_r }
    }

    fn permute_l(&mut self, n: usize) {
        self.alpha_l.rotate_left(n);
        let t = self.alpha_l.remove(1).unwrap();
        self.alpha_l.insert(13, t);
    }

    fn permute_r(&mut self, n: usize) {
        self.alpha_r.rotate_left(n+1);
        let t = self.alpha_r.remove(2).unwrap();
        self.alpha_r.insert(13, t);
    }

    pub fn encrypt(&mut self, text: &str) -> String {
        let symbols = text.chars();
        let mut out = String::new();
        for c in symbols {
            let n = self.alpha_l.iter().position(|x| x == &c).unwrap();
            out.push(self.alpha_r[n]);
            self.permute_l(n);
            self.permute_r(n);
        }
        out

    }

    fn decrypt(&mut self, text: &str) -> String {
        let symbols = text.chars();
        let mut out = String::new();
        for c in symbols {
            let n = self.alpha_r.iter().position(|x| x == &c).unwrap();
            out.push(self.alpha_l[n]);
            self.permute_l(n);
            self.permute_r(n);
        }
        out
        
    }

}

/* impl fmt::Display for Chaocipher<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Chaocipher Cipher\nkey: {}",self.key_name)
    }
} */

#[test]
fn test_chaociper() {
    let mut c = Chaocipher::new("HXUCZVAMDSLKPEFJRIGTWOBNYQ","PTLNBQDEOYSFAVZKGJRIHWXUMC");

    println!("{}",c.decrypt("WELLDONEISBETTERTHANWELLSAID"));
}