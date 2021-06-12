use std::fmt;
use std::collections::VecDeque;
use std::iter::FromIterator;

fn permute_l(alpha: &mut VecDeque<char>, n: usize) {
    alpha.rotate_left(n);
    let t = alpha.remove(1).unwrap();
    alpha.insert(13, t);
}

fn permute_r(alpha: &mut VecDeque<char>, n: usize) {
    alpha.rotate_left(n+1);
    let t = alpha.remove(2).unwrap();
    alpha.insert(13, t);
}


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
}

impl crate::Cipher for Chaocipher {

    fn encrypt(&self, text: &str) -> String {
        let mut left = self.alpha_l.clone();
        let mut right = self.alpha_r.clone();

        let symbols = text.chars();
        let mut out = String::new();
        for c in symbols {
            let n = right.iter().position(|x| x == &c).unwrap();
            out.push(left[n]);
            permute_l(&mut left,n);
            permute_r(&mut right,n);
        }
        
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let mut left = self.alpha_l.clone();
        let mut right = self.alpha_r.clone();

        let symbols = text.chars();
        let mut out = String::new();
        for c in symbols {
            let n = left.iter().position(|x| x == &c).unwrap();
            out.push(right[n]);
            permute_l(&mut left,n);
            permute_r(&mut right,n);
        }
        
        out

    }

}

impl fmt::Display for Chaocipher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let l: String = self.alpha_l.iter().collect();
        let r: String = self.alpha_r.iter().collect();
        write!(f, "Chaocipher Cipher\n{}\n{}",l,r)
    }
}