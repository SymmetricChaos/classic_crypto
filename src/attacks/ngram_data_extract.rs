use serde::Deserialize;
use csv::Writer;
use std::error::Error;

// Assign a String a numerical (f64) score between 0 and 1 estimating how likely it is to be English text


// open 1grams.csv
// add up all the counts as a u64 to get a total
// close 1grams.csv
// open 1grams.csv (again)
// use the total to calculate the probability and log probability of seeing each letter
// store that information in a Vec<(String,f64,f64)>
// close 1grams.csv
// open 1gram_scores.csv
// write in the letters information from the Vec<(String,f64,f64)>
// close 1gram_scores.csv

#[derive(Debug, Deserialize)]
struct Row {
    ngram: String,
    count: f64
}

fn score_1_grams() -> Result<(), Box<dyn Error>> {
    let mut total = 0f64;
    let mut rdr = csv::Reader::from_path("src\\attacks\\1grams.csv")?;
    for result in rdr.deserialize() {
        // Conver the record to a Row struct
        let record: Row = result?;
        total += record.count
    }
    let normalizer = 1.0/total;
    let mut wtr = Writer::from_path("src\\attacks\\1grams_data.csv")?;
    for result in rdr.deserialize() {
        // Conver the record to a Row struct
        let record: Row = result?;
        wtr.write_record(&[record.ngram,
                           record.count.to_string(),
                           (record.count*normalizer).to_string()])?;
    }
    Ok(())
}




fn main() {
    score_1_grams();
}