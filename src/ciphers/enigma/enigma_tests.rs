

#[cfg(test)]
mod enigma_tests {
    use crate::ciphers::enigma::{ROTORS,REFLECTORS,Plugboard,EnigmaM3};

    fn char_to_usize(c: char) -> usize {
        (c as u8 as usize) - 65
    }
    
    fn usize_to_char(n: usize) -> char {
        (n + 65) as u8 as char
    }

    #[test]
    fn plugboard() {
        let board = Plugboard::new("AB XY");

        assert_eq!('A', board.swap('B'));
        assert_eq!('B', board.swap('A'));
        assert_eq!('C', board.swap('C'));
        assert_eq!('X', board.swap('Y'));
        
    }

    #[test]
    fn single_rotor() {
        let rotor = ROTORS["III"];
        let c = char_to_usize('A');
        assert_eq!('B', usize_to_char(rotor.encrypt_rtl(c)));
    }

    #[test]
    fn single_rotor_stepping() {
        let mut rotor = ROTORS["III"];
        println!("{}",rotor);
        rotor.set_ring(1);
        rotor.set_position(0);

        rotor.step();
        assert_eq!(
            'B',
            usize_to_char(rotor.encrypt_rtl(0)));
        
        rotor.step();
        assert_eq!(
            'C',
            usize_to_char(rotor.encrypt_rtl(0)));
        
        rotor.step();
        assert_eq!(
            'D',
            usize_to_char(rotor.encrypt_rtl(0)));

        rotor.step();
        assert_eq!(
            'E',
            usize_to_char(rotor.encrypt_rtl(0)));
    }

    #[test]
    fn single_rotor_stepping_2() {
        let mut rotor = ROTORS["II"];
        rotor.set_ring(25);
        rotor.set_position(25);
        
        rotor.step();
        assert_eq!('I',usize_to_char(rotor.encrypt_rtl(0)));
        
        rotor.step();
        assert_eq!('B',usize_to_char(rotor.encrypt_rtl(0)));
        
        rotor.step();
        assert_eq!('H', usize_to_char(rotor.encrypt_rtl(0)));

        rotor.step();
        assert_eq!('O',usize_to_char(rotor.encrypt_rtl(0)));
    }

    #[test]
    fn single_rotor_indexer() {
        let mut rotor = ROTORS["II"];
        rotor.set_position(24);
        assert_eq!("AJDKSIRUXBLHWTMCQGZNPYFVO͓E",format!("{}",rotor));
        rotor.step();
        assert_eq!("AJDKSIRUXBLHWTMCQGZNPYFVOE͓",format!("{}",rotor));
        rotor.step();
        assert_eq!("A͓JDKSIRUXBLHWTMCQGZNPYFVOE",format!("{}",rotor));
        rotor.step();
        assert_eq!("AJ͓DKSIRUXBLHWTMCQGZNPYFVOE",format!("{}",rotor));
    }

    #[test]
    fn enigma() {

        let rotors = (ROTORS["IV"], ROTORS["II"], ROTORS["V"]);
        let reflector = REFLECTORS["B"];
        let ring_positions = (14,22,25);

        let mut s = EnigmaM3::new( "EJ OY IV AQ KW FX MT PS LU BD", rotors, reflector, ring_positions );

        //println!("{}",s);
        /*
        Enigma M3
        Plugboard: EJ OY IV AQ KW FX MT PS LU BD
        Rotor 1: E͓SOVPZJAYQUIRHXLNFTGKDCMWB (14)
        Rotor 2: A͓JDKSIRUXBLHWTMCQGZNPYFVOE (22)
        Rotor 3: V͓ZBRGITYUPSDNHLXAWMJQOFECK (25)
        Reflector: YRUHQSLDPXNGOKMIEBFZCWVJAT
        */

        s.set_rotors((0,0,0));
        let text = "AAAAAAAAAAAAAAAAAAAAAAAAAA";
        let out = s.encrypt(text);
        assert_eq!(&out,"VDDXSYJOVCQYJSDJMLONNSSJQI");

        s.set_rotors((0,0,0));
        // Confirm involution property
        let text = "VDDXSYJOVCQYJSDJMLONNSSJQI";
        let out = s.encrypt(text);
        assert_eq!(&out,"AAAAAAAAAAAAAAAAAAAAAAAAAA");
    }
}