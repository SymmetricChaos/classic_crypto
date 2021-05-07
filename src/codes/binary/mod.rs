/// Various Codes

pub mod bacon;
pub mod fibonacci;
pub mod morse;
pub mod baudot;
mod code_generators;

pub use self::bacon::Bacon;
pub use self::morse::MorseITU;
pub use self::fibonacci::Fibonacci;
pub use self::baudot::BaudotITA2;