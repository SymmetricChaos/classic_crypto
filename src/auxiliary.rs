use std::ops::Shr;



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
    text.chars().map(|c| alphabet.chars().position(|x| x == c).unwrap() / 2 ).collect()
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







#[test]
fn check_ranker() {

    assert_eq!(rank_str("ACDC","ABCDEFGHIJKLMNOPQRSTUVWXYZ"),vec![0,1,3,2]);

}


