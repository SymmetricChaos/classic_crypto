// Assign a String a numerical (f64) score between 0 and 1 estimating how likely it is to be English text

use std::error::Error;
use std::collections::HashMap;

use itertools::Itertools;

fn read_1grams() -> HashMap<String,f64> {

    let mut map: HashMap<String,f64> = HashMap::new();

    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path("src\\attacks\\1gramScores.csv").unwrap();
    for result in rdr.records() {
        // Conver the record to a string
        let record: String = result.unwrap().deserialize(None).unwrap();
        // Convert the string to a vector
        let rec_vec =  record.split(' ').collect_vec();
        map.insert(rec_vec[0].to_string(), rec_vec[3].parse().unwrap());
    }

    map
}



/* pub fn score_text(text: &str) -> f64 {

} */