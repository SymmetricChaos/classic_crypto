use crate::ciphers::enigma::{ROTOR,REFLECTOR,Plugboard,EnigmaM3};

fn char_to_usize(c: char) -> usize {
    (c as u8 as usize) - 65
}

fn usize_to_char(n: usize) -> char {
    (n + 65) as u8 as char
}

#[test]
fn plugboard() {
    let board = Plugboard::new("AB XY");

    println!("{} -> {}",'A',board.swap('A'));
    println!("{} -> {}",'B',board.swap('B'));
    println!("{} -> {}",'C',board.swap('C'));
}

#[test]
fn single_rotor() {
    let rotor = ROTOR["III"];
    println!("{}",rotor);
    let c = char_to_usize('A');
    println!("{} -> {}", 'A', usize_to_char(rotor.encrypt_rtl(c)));
    println!("should get: A -> B")
}

#[test]
fn single_rotor_stepping() {
    let mut rotor = ROTOR["III"];
    println!("{}",rotor);
    rotor.set_ring(1);
    rotor.set_position(0);

    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encrypt_rtl(0)));
    
    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encrypt_rtl(0)));
    
    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encrypt_rtl(0)));

    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encrypt_rtl(0)));

    println!("right column: BCDE\nleft column: BCDE")
}

#[test]
fn single_rotor_stepping_2() {
    let mut rotor = ROTOR["II"];
    println!("{}",rotor);
    rotor.set_ring(25);
    rotor.set_position(25);

    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encrypt_rtl(0)));
    
    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encrypt_rtl(0)));
    
    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encrypt_rtl(0)));

    rotor.step();
    println!("{}  {}",
        usize_to_char(rotor.get_position()),
        usize_to_char(rotor.encrypt_rtl(0)));

    println!("right column: ABCD\nleft column: IBHO")
}

#[test]
fn enigma() {
    let rotor1 = ROTOR["IV"];
    let rotor2 = ROTOR["II"];
    let rotor3 = ROTOR["V"];
    let rotors = (rotor1, rotor2, rotor3);
    let reflector = REFLECTOR["B"];
    let ring_positions = (14,22,25);

    let mut s = EnigmaM3::new( "EJ OY IV AQ KW FX MT PS LU BD", rotors, reflector, ring_positions );

    println!("\n{}\n",s);

    s.set_rotors((0,0,0));
    let text = "AAAAAAAAAAAAAAAAAAAAAAAAAA";
    let out = s.encrypt(text);
    println!("{}\n{}",text,out);
    assert_eq!(&out,"VDDXSYJOVCQYJSDJMLONNSSJQI");

    // Confirm involution property
    let text = "VDDXSYJOVCQYJSDJMLONNSSJQI";
    let out = s.encrypt(text);
    assert_eq!(&out,"AAAAAAAAAAAAAAAAAAAAAAAAAA");
}