// Assign a String a numerical (f64) score between 0 and 1 estimating how likely it is to be English text

use lazy_static::lazy_static;
use std::collections::HashMap;

use itertools::Itertools;


lazy_static! {
    pub static ref MONOGRAMS: HashMap<char, f64> = {
        let mut map: HashMap<char, f64> = HashMap::new();
        let mut rdr = csv::Reader::from_path("src\\attacks\\1gramScores.csv").unwrap();
        for result in rdr.records() {
            // Conver the record to a string
            let record: String = result.unwrap().deserialize(None).unwrap();
            // Convert the string to a vector
            let rec_vec =  record.split(' ').collect_vec();
            map.insert(rec_vec[0].chars().next().unwrap(), rec_vec[3].parse().unwrap());
        }
        map
    };
}

pub fn score_text(text: &str) -> f64 {
    let mut score = 0f64;
    for c in text.chars() {
        score += MONOGRAMS[&c]
    }
    score
}

#[cfg(test)]
mod text_scoring_tests {
    use super::*;

    #[test]
    fn letter_score() {
        println!("{}",score_text("THEFOX"))
    }
}