use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref ALPHA: HashMap<u8,u8> = {
        let mut m = HashMap::new();
        for n in 0..26 {
            m.insert(n, n);
            m.insert(n+65, n);
            m.insert(n+97, n);
        }
        m
    };
}