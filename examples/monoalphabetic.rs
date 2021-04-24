use classic_crypto::ciphers::{Caesar,Affine};
use classic_crypto::errors::CipherError;
use classic_crypto::alphabet::ALPHA26;

fn main() -> Result<(),CipherError> {

    let plaintext = "It was the best of times, it was the worst of times, it was the age of wisdom, it was the age of foolishness, it was the epoch of belief, it was the epoch of incredulity, it was the season of Light, it was the season of Darkness, it was the spring of hope, it was the winter of despair.";
    let plaintext_nospace = "ITWASTHEBESTOFTIMESITWASTHEWORSTOFTIMESITWASTHEAGEOFWISDOMITWASTHEAGEOFFOOLISHNESSITWASTHEEPOCHOFBELIEFITWASTHEEPOCHOFINCREDULITYITWASTHESEASONOFLIGHTITWASTHESEASONOFDARKNESSITWASTHESPRINGOFHOPEITWASTHEWINTEROFDESPAIR";
    println!("Our example plaintext is from Dickens and contains a lot of repetition:\n{}",plaintext_nospace);

    println!("\n\nThe repetitiveness of the plaintext helps to highlight the main weakness of monoalphabetic subtitution ciphers, their failure to disguise any patterns in the text.");

    let caesar = Caesar::new(1, ALPHA26.clone());
    println!("\n{}",caesar);
    let ciphertext = caesar.encode(plaintext)?;
    println!("{}",ciphertext);

    let affine = Affine::new((2,3), ALPHA26.clone());
    println!("\n{}",affine);
    let ciphertext = affine.encode(plaintext)?;
    println!("{}",ciphertext);


    Ok(())
}