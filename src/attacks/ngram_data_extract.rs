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

fn data_from_1grams() -> Result<(), Box<dyn Error>> {
    let mut total = 0f64;
    let mut rows = Vec::new();
    let mut rdr = csv::Reader::from_path("src\\attacks\\1grams.csv")?;
    for result in rdr.deserialize() {
        // Conver the record to a Row struct
        let record: Row = result?;
        total += record.count;
        rows.push(record)
    }
    let normalizer = 1.0/total;
    let mut wtr = Writer::from_path("src\\attacks\\1grams_data.csv")?;
    wtr.write_record(&["ngram","count","normed"])?;
    for row in rows {
        wtr.write_record(&[row.ngram,
                           row.count.to_string(),
                           (row.count*normalizer).to_string()])?;
    }
    Ok(())
}

fn data_from_2grams() -> Result<(), Box<dyn Error>> {
    let mut total = 0f64;
    let mut rows = Vec::new();
    let mut rdr = csv::Reader::from_path("src\\attacks\\2grams.csv")?;
    for result in rdr.deserialize() {
        // Conver the record to a Row struct
        let record: Row = result?;
        total += record.count;
        rows.push(record)
    }
    let normalizer = 1.0/total;
    let mut wtr = Writer::from_path("src\\attacks\\2grams_data.csv")?;
    wtr.write_record(&["ngram","count","normed"])?;
    for row in rows {
        wtr.write_record(&[row.ngram,
                           row.count.to_string(),
                           (row.count*normalizer).to_string()])?;
    }
    Ok(())
}

fn data_from_3grams() -> Result<(), Box<dyn Error>> {
    let mut total = 0f64;
    let mut rows = Vec::new();
    let mut rdr = csv::Reader::from_path("src\\attacks\\3grams.csv")?;
    for result in rdr.deserialize() {
        // Conver the record to a Row struct
        let record: Row = result?;
        total += record.count;
        rows.push(record)
    }
    let normalizer = 1.0/total;
    let mut wtr = Writer::from_path("src\\attacks\\3grams_data.csv")?;
    wtr.write_record(&["ngram","count","normed"])?;
    for row in rows {
        wtr.write_record(&[row.ngram,
                           row.count.to_string(),
                           (row.count*normalizer).to_string()])?;
    }
    Ok(())
}

fn data_from_4grams() -> Result<(), Box<dyn Error>> {
    let mut total = 0f64;
    let mut rows = Vec::new();
    let mut rdr = csv::Reader::from_path("src\\attacks\\4grams.csv")?;
    for result in rdr.deserialize() {
        // Conver the record to a Row struct
        let record: Row = result?;
        total += record.count;
        rows.push(record)
    }
    let normalizer = 1.0/total;
    let mut wtr = Writer::from_path("src\\attacks\\4grams_data.csv")?;
    wtr.write_record(&["ngram","count","normed"])?;
    for row in rows {
        wtr.write_record(&[row.ngram,
                           row.count.to_string(),
                           (row.count*normalizer).to_string()])?;
    }
    Ok(())
}


#[test]
fn main() {
    data_from_1grams();
    data_from_2grams();
    data_from_3grams();
    data_from_4grams();
}