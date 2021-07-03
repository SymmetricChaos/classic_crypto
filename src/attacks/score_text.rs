// Assign a String a numerical (f64) score between 0 and 1 estimating how likely it is to be English text

use lazy_static::lazy_static;
use std::collections::HashMap;
use itertools::Itertools;
use std::convert::TryFrom;
use serde::Deserialize;

use crate::alphabets::LATIN26;

#[derive(Debug, Deserialize)]
struct DataRow {
    ngram: String,
    count: f64,
    normed: f64,
}

lazy_static! {
    pub static ref MONOGRAMS: HashMap<char, f64> = {
        let mut map: HashMap<char, f64> = HashMap::new();
        let mut rdr = csv::Reader::from_path("src\\attacks\\1grams_data.csv").unwrap();
        for result in rdr.deserialize() {
            // Conver the record to a Row struct
            let record: DataRow = result.unwrap();
            // Convert the string to a vector
            map.insert(record.ngram.chars().next().unwrap(),record.normed);
        }
        map
    };
}

pub fn score_text(text: &str) -> f64 {

    let mut empirical: HashMap<char, f64> = HashMap::new();
    for l in LATIN26.chars() {
        empirical.insert(l, 0.0);
    }

    let total = text.chars().count() as f64;
    let step = 1.0/total;

    for c in text.chars() {
        *empirical.get_mut(&c).unwrap() += step
    }

    let mut score = 0f64;
    for l in LATIN26.chars() {
        score += (MONOGRAMS[&l]-empirical[&l]).abs()
    }

    score
}

#[cfg(test)]
mod text_scoring_tests {
    use super::*;

    #[test]
    fn letter_score() {
        println!("{}",score_text("THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG"))
    }
}