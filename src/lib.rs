//! Simple ansi colors
//! ```rust,no_run
//! use scolor::Color;
//! println!("{}", "hello".red());
//! println!("{}", "world".green());
//! println!("{}", "!".rgb(123,12,50));
//! ```
use std::fmt::Display;

pub trait Color {
    fn rgb(&self, r: u8, g: u8, b: u8) -> ColorString;
    fn red(&self) -> ColorString;
    fn green(&self) -> ColorString;
    fn yellow(&self) -> ColorString;
    fn blue(&self) -> ColorString;
    fn light_blue(&self) -> ColorString;
}

macro_rules! cs {
    ($string:expr, $r:expr,$g:expr,$b:expr) => {
        ColorString {
            string: $string,
            color: Rgb($r, $g, $b),
        }
    };
}

impl<T> Color for T
where
    T: Display,
{
    fn rgb(&self, r: u8, g: u8, b: u8) -> ColorString {
        cs!(self, r, g, b)
    }
    fn red(&self) -> ColorString {
        cs!(self, 255, 0, 0)
    }
    fn green(&self) -> ColorString {
        cs!(self, 0, 255, 0)
    }
    fn yellow(&self) -> ColorString {
        cs!(self, 255, 255, 0)
    }
    fn blue(&self) -> ColorString {
        cs!(self, 0, 0, 255)
    }
    fn light_blue(&self) -> ColorString {
        cs!(self, 0, 150, 255)
    }
}

#[doc(hidden)]
pub struct ColorString<'a> {
    string: &'a dyn Display,
    color: Rgb,
}

impl Display for ColorString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const DEL1: &str = "\x1b[";
        const DEL2: &str = "m";
        const DEL3: &str = "\x1b[0m";
        const FG: &str = "38";
        write!(
            f,
            "{}{};2;{}{}{}{}",
            DEL1, FG, self.color, DEL2, self.string, DEL3
        )
    }
}

struct Rgb(u8, u8, u8);

impl std::fmt::Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};{};{}", self.0, self.1, self.2)
    }
}
