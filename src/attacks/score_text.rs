// Assign a String a numerical (f64) score between 0 and 1 estimating how likely it is to be English text

use std::error::Error;

#[test]
fn example() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path("1gramScores.csv").unwrap();
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}



/* pub fn score_text(text: &str) -> f64 {

} */