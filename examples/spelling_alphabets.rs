use classic_crypto::codes::SpellingAlphabet;

//
fn main()  {

    println!("The NATO Spelling Alphabet");
    let plaintext = "ABC123";
    let nato = SpellingAlphabet::nato();
    let ccb = SpellingAlphabet::ccb();
    let nato_coded = nato.encode(plaintext);
    let ccb_coded = ccb.encode(plaintext);
    println!("{}\nNATO: {}\nCCB: {}",plaintext,nato_coded,ccb_coded);

}