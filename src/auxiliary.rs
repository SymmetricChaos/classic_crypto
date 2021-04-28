pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub const LATIN25_J: &str = "ABCDEFGHIKLMNOPQRSTUVWXYZ";
pub const LATIN25_Q: &str = "ABCDEFGHIJKLMNOPRSTUVWXYZ";
pub const LATIN26: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const LATIN36: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub fn keyed_alphabet(keyword: &str, alphabet: &str) -> String {
    let mut keyed_alpha = "".to_string();
    for k in keyword.chars() {
        let ks = &k.to_string();
        if !alphabet.contains(ks) {
            panic!("keyword must use symbols from the alphabet: {}",alphabet)
        }
        if keyed_alpha.contains(ks) {
            continue
        } else {
            keyed_alpha.push(k)
        }
    }

    for a in alphabet.chars() {
        if keyed_alpha.contains(&a.to_string()) {
            continue
        } else {
            keyed_alpha.push(a)
        }
    }
    keyed_alpha
}