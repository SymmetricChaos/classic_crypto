//! Ciphers based on the Playfair Cipher
mod playfair;
mod two_square;
mod four_square;
mod slidefair;
mod seriated_playfair;

mod playfair_tests;

pub use self::playfair::Playfair;
pub use self::two_square::{TwoSquare,TwoSquareInverting};
pub use self::four_square::FourSquare;
pub use self::slidefair::Slidefair;
pub use self::seriated_playfair::SeriatedPlayfair;