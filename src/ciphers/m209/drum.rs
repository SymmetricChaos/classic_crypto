use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};

#[derive(Clone,Debug)]
pub struct Drum<'a> {
    bars: Vec<(usize,usize)>
}


impl Drum<'_> {
    pub fn new(bars: Vec<(usize,usize)>) -> Drum {
        Drum{ bars }
    }

    // The Drum turns through all 27 positions
    pub fn revolve(&self) {

    }

}