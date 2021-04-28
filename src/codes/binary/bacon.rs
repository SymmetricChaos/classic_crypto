use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref BACON_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("A","00000");
        m.insert("B","00001");
        m.insert("C","00010");
        m.insert("D","00011");
        m.insert("E","00100");
        m.insert("F","00101");
        m.insert("G","00110");
        m.insert("H","00111");
        m
    };

}

pub struct Bacon {
    map: HashMap<&'static str,&'static str>,
    map_inv: HashMap<&'static str,&'static str>,
}

