
use std::{ fs::File, io::{Write,Error, Read}};

use classic_crypto::ciphers::{Enigma,Plugboard,prep_file,ENIGMA_ROTORS,ENIGMA_REFLECTORS};

fn main() -> Result<(),Error> {

    prep_file("sample_text.txt","enigma_plaintext")?;

    let plugboard = Plugboard::new("EJ OY IV AQ KW FX MT PS LU BD");
    let rotor1 = ENIGMA_ROTORS["IV"].clone();
    let rotor2 = ENIGMA_ROTORS["II"].clone();
    let rotor3 = ENIGMA_ROTORS["V"].clone();
    let rotors = (rotor1, rotor2, rotor3);
    let reflector = ENIGMA_REFLECTORS["B"].clone();
    let ring_positions = (14,22,25);

    let mut s = Enigma::new( plugboard, rotors, reflector, ring_positions );

    let target_name = format!("{}.txt","engima_ciphertext");
    let mut target_file = File::create(target_name)?;

    let mut source_file = File::open("enigma_plaintext.txt")?;
    let mut source_text = String::new();
    source_file.read_to_string(&mut source_text)?;

    let ciphertext = s.encode(&source_text, (0,0,0));

    target_file.write(ciphertext.as_bytes())?;

    Ok(())
}