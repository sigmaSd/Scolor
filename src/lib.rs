//! Simple ansi colors
//! ```rust,no_run
//! use scolor::{Color, ColorExt};
//!
//! println!("{}", "hello".red().bold().underline());
//! println!("{}", "world".green().red_bg().italic());
//! println!("{}", "!".rgb(123,12,50));
//! ```
// Credits to https://stackoverflow.com/a/33206814
use std::fmt::Display;

pub trait Color {
    fn rgb(&self, r: u8, g: u8, b: u8) -> ColorString;
    fn rgb_bg(&self, r: u8, g: u8, b: u8) -> ColorString;
    fn style(&self, effect: Effect) -> ColorString;
}

pub trait ColorExt {
    fn red(&self) -> ColorString;
    fn red_bg(&self) -> ColorString;
    fn green(&self) -> ColorString;
    fn green_bg(&self) -> ColorString;
    fn yellow(&self) -> ColorString;
    fn yellow_bg(&self) -> ColorString;
    fn blue(&self) -> ColorString;
    fn blue_bg(&self) -> ColorString;
    fn light_blue(&self) -> ColorString;
    fn light_blue_bg(&self) -> ColorString;
    fn italic(&self) -> ColorString;
    fn bold(&self) -> ColorString;
    fn underline(&self) -> ColorString;
    fn crossed_out(&self) -> ColorString;
}

impl<T> Color for T
where
    T: Display,
{
    fn rgb(&self, r: u8, g: u8, b: u8) -> ColorString {
        ColorString {
            string: self,
            color: Box::new(FG(r, g, b)),
        }
    }

    fn rgb_bg(&self, r: u8, g: u8, b: u8) -> ColorString {
        ColorString {
            string: self,
            color: Box::new(BG(r, g, b)),
        }
    }

    fn style(&self, effect: Effect) -> ColorString {
        ColorString {
            string: self,
            color: Box::new(effect),
        }
    }
}

impl<T> ColorExt for T
where
    T: Color,
{
    fn red(&self) -> ColorString {
        self.rgb(255, 0, 0)
    }
    fn red_bg(&self) -> ColorString {
        self.rgb_bg(255, 0, 0)
    }
    fn green(&self) -> ColorString {
        self.rgb(0, 255, 0)
    }
    fn green_bg(&self) -> ColorString {
        self.rgb_bg(0, 255, 0)
    }
    fn yellow(&self) -> ColorString {
        self.rgb(255, 255, 0)
    }
    fn yellow_bg(&self) -> ColorString {
        self.rgb_bg(255, 255, 0)
    }
    fn blue(&self) -> ColorString {
        self.rgb(0, 0, 255)
    }
    fn blue_bg(&self) -> ColorString {
        self.rgb_bg(0, 0, 255)
    }
    fn light_blue(&self) -> ColorString {
        self.rgb(0, 150, 255)
    }
    fn light_blue_bg(&self) -> ColorString {
        self.rgb_bg(0, 150, 255)
    }
    fn italic(&self) -> ColorString {
        self.style(Effect::Italic)
    }
    fn bold(&self) -> ColorString {
        self.style(Effect::Bold)
    }
    fn underline(&self) -> ColorString {
        self.style(Effect::Underline)
    }
    fn crossed_out(&self) -> ColorString {
        self.style(Effect::CrossedOut)
    }
}

pub enum Effect {
    Bold,
    Faint,
    Italic,
    Underline,
    ReverseVideo,
    CrossedOut,
}

impl std::fmt::Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Effect::Bold => write!(f, "1"),
            Effect::Faint => write!(f, "2"),
            Effect::Italic => write!(f, "3"),
            Effect::Underline => write!(f, "4"),
            Effect::ReverseVideo => write!(f, "7"),
            Effect::CrossedOut => write!(f, "9"),
        }
    }
}

#[doc(hidden)]
pub struct ColorString<'a> {
    string: &'a dyn Display,
    color: Box<dyn Display>,
}

impl Display for ColorString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const START_DEL: &str = "\x1b[";
        const COLOR_END_DEL: &str = "m";
        const END_DEL: &str = "\x1b[0m";
        write!(f, "{}", START_DEL)?;
        write!(f, "{}", self.color)?;
        write!(f, "{}", COLOR_END_DEL)?;
        write!(f, "{}", self.string)?;
        write!(f, "{}", END_DEL)
    }
}

struct FG(u8, u8, u8);

impl std::fmt::Display for FG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const FG_DEL: &str = "38;2";
        write!(f, "{};{};{};{}", FG_DEL, self.0, self.1, self.2)
    }
}
struct BG(u8, u8, u8);

impl std::fmt::Display for BG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const BG_DEL: &str = "48;2";
        write!(f, "{};{};{};{}", BG_DEL, self.0, self.1, self.2)
    }
}
