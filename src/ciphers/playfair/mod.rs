//! Ciphers based on the Playfair Cipher
pub mod playfair;
pub mod two_square;
pub mod four_square;

pub use self::playfair::Playfair;
pub use self::two_square::TwoSquare;