use classic_crypto::codes::SpellingAlphabet;
use classic_crypto::Code;

//
fn main()  {

    println!("Example Spelling Alphabets");
    let plaintext = "ABC123";
    let nato = SpellingAlphabet::nato();
    let ccb = SpellingAlphabet::ccb();
    let nato_coded = nato.encode(plaintext);
    let ccb_coded = ccb.encode(plaintext);
    println!("{}\nNATO: {}\nCCB: {}",plaintext,nato_coded,ccb_coded);

}