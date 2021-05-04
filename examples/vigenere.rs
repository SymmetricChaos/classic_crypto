use classic_crypto::ciphers::{Vigenere,Autokey};
use classic_crypto::alphabets::LATIN26;

//
fn main()  {

    let plaintext = "ITWASTHEBESTOFTIMESITWASTHEWORSTOFTIMESITWASTHEAGEOFWISDOMITWASTHEAGEOFFOOLISHNESSITWASTHEEPOCHOFBELIEFITWASTHEEPOCHOFINCREDULITYITWASTHESEASONOFLIGHTITWASTHESEASONOFDARKNESSITWASTHESPRINGOFHOPEITWASTHEWINTEROFDESPAIR";
    println!("Our example ciphertext is from Dickens and contains a lot of repetition\n{}",plaintext);


    let vigenere = Vigenere::new("SECRET", LATIN26);
    println!("\n\n{}",vigenere);
    let ciphertext = vigenere.encrypt(plaintext);
    let decrypted = vigenere.decrypt(&ciphertext);
    println!("{}",ciphertext);
    assert_eq!(decrypted,plaintext);

    let autokey = Autokey::new("SECRET", LATIN26);
    println!("\n\n{}",autokey);
    let ciphertext = autokey.encrypt(plaintext);
    let decrypted = autokey.decrypt(&ciphertext);
    println!("{}",ciphertext);
    assert_eq!(decrypted,plaintext);

}