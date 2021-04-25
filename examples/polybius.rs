use classic_crypto::ciphers::polybius::{Polybius,ADFGVX};


fn main()  {

    let nospace = "ITWASTHEBESTOFTIMESITWASTHEWORSTOFTIMESITWASTHEAGEOFWISDOMITWASTHEAGEOFFOOLISHNESSITWASTHEEPOCHOFBELIEFITWASTHEEPOCHOFINCREDULITYITWASTHESEASONOFLIGHTITWASTHESEASONOFDARKNESSITWASTHESPRINGOFHOPEITWASTHEWINTEROFDESPAIR";
    println!("Our example ciphertext is from Dickens and contains a lot of repetition\n{}",nospace);


    let polybius = Polybius::new("0AB1CD2EF3GH4IJ5KL6MN7OP8QR9STUVWXYZ","123456");
    println!("\n\n{}",polybius);
    let ciphertext = polybius.encode(nospace);
    let cleartext = polybius.decode(&ciphertext);
    println!("{}",ciphertext);
    assert_eq!(cleartext,nospace);

    let adfgvx = ADFGVX::new("0AB1CD2EF3GH4IJ5KL6MN7OP8QR9STUVWXYZ",vec![1,0,3,2,5,4]);
    println!("\n\n{}",adfgvx);
    let ciphertext = adfgvx.encode(nospace);
    //let cleartext = adfgvx.decode(&ciphertext);
    println!("{}",ciphertext);
    //assert_eq!(cleartext,nospace+);

}