use classic_crypto::ciphers::polybius::Polybius;


fn main()  {

    let nospace = "ITWASTHEBESTOFTIMESITWASTHEWORSTOFTIMESITWASTHEAGEOFWISDOMITWASTHEAGEOFFOOLISHNESSITWASTHEEPOCHOFBELIEFITWASTHEEPOCHOFINCREDULITYITWASTHESEASONOFLIGHTITWASTHESEASONOFDARKNESSITWASTHESPRINGOFHOPEITWASTHEWINTEROFDESPAIR";
    println!("Our example ciphertext is from Dickens and contains a lot of repetition\n{}",nospace);

    println!("The Polybius Square dates back to Ancient Greece where it was originally proposed as a way to send singals. The alphabet it written into a grid with the sides labeled. Each symbol is then mapped to a unique pair of numbers. Below is an example Polybius square on a 6x6 grid with the Latin alphabet and Indo-Arabic numerals.");

    let polybius = Polybius::new("IHAVE17ZEBRASAND293HORSES", "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789", "123456");
    println!("\n\n{}",polybius);

    println!("\nThe letter 'A' would encode as the pair (1,3) while 'D' would encode as (3,1). When used in encryption these pairs are generally read off as 13 and 31.\nOn its own the Polybius Square provides no more encryption than a simple substitution cipher but can form the basis of much stronger ciphers when combined with other techniques.");

}