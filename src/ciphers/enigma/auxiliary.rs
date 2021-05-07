use std::{ fs::File, io::{Write,Error, Read}};

pub fn prep_text(text: &str) -> String {
    if !text.is_ascii() {
        panic!("Non-ASCII characters are forbidden.")
    } else {
        let upper = text.to_ascii_uppercase();
        let mut out = "".to_string();
        for word in upper.split(|x: char| !x.is_ascii_alphabetic()) {
            out.push_str(word);
        }
        out
    }
}

pub fn prep_file(source: &str, target: &str) -> Result<(),Error> {
    let mut target_file = File::create(target.to_string())?;

    let mut source_file = File::open(source)?;
    let mut source_text = String::new();
    source_file.read_to_string(&mut source_text)?;

    let clean_text = prep_text( &source_text);

    target_file.write(clean_text.as_bytes())?;

    Ok(())
}
