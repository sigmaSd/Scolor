//! Simple ansi colors
//! ```rust,no_run
//! use scolor::ColorExt;
//!
//! println!("{}", "hello".red().bold().underline());
//! println!("{}", "world".green().red_bg().italic());
//! ```
// Credits to https://stackoverflow.com/a/33206814
use std::fmt::Display;

pub trait Color {
    fn rgb(&self, r: u8, g: u8, b: u8, ctype: ColorType) -> ColorString;
    fn style(&self, effect: Effect) -> ColorString;
}

pub trait ColorExt {
    fn rgb_fg(&self, r: u8, g: u8, b: u8) -> ColorString;
    fn rgb_bg(&self, r: u8, g: u8, b: u8) -> ColorString;
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
    fn rgb(&self, r: u8, g: u8, b: u8, ctype: ColorType) -> ColorString {
        ColorString {
            string: self,
            color: Some(Rgb { r, g, b, ctype }),
            effect: None,
        }
    }
    fn style(&self, effect: Effect) -> ColorString {
        ColorString {
            string: self,
            color: None,
            effect: Some(effect),
        }
    }
}

impl<T> ColorExt for T
where
    T: Color,
{
    fn rgb_fg(&self, r: u8, g: u8, b: u8) -> ColorString {
        self.rgb(r, g, b, ColorType::Fg)
    }
    fn rgb_bg(&self, r: u8, g: u8, b: u8) -> ColorString {
        self.rgb(r, g, b, ColorType::Bg)
    }
    fn red(&self) -> ColorString {
        self.rgb_fg(255, 0, 0)
    }
    fn red_bg(&self) -> ColorString {
        self.rgb_bg(255, 0, 0)
    }
    fn green(&self) -> ColorString {
        self.rgb_fg(0, 255, 0)
    }
    fn green_bg(&self) -> ColorString {
        self.rgb_bg(0, 255, 0)
    }
    fn yellow(&self) -> ColorString {
        self.rgb_fg(255, 255, 0)
    }
    fn yellow_bg(&self) -> ColorString {
        self.rgb_bg(255, 255, 0)
    }
    fn blue(&self) -> ColorString {
        self.rgb_fg(0, 0, 255)
    }
    fn blue_bg(&self) -> ColorString {
        self.rgb_bg(0, 0, 255)
    }
    fn light_blue(&self) -> ColorString {
        self.rgb_fg(0, 150, 255)
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

pub enum ColorType {
    Fg,
    Bg,
}

struct Rgb {
    r: u8,
    g: u8,
    b: u8,
    ctype: ColorType,
}

impl std::fmt::Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const FG_DEL: &str = "38;2";
        const BG_DEL: &str = "48;2";
        let del = match self.ctype {
            ColorType::Fg => FG_DEL,
            ColorType::Bg => BG_DEL,
        };
        write!(f, "{};{};{};{}", del, self.r, self.g, self.b)
    }
}

#[doc(hidden)]
pub struct ColorString<'a> {
    string: &'a dyn Display,
    color: Option<Rgb>,
    effect: Option<Effect>,
}

impl Display for ColorString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const START_DEL: &str = "\x1b[";
        const COMPONENT_DEL: &str = ";";
        const COLOR_END_DEL: &str = "m";
        const END_DEL: &str = "\x1b[0m";

        write!(f, "{}", START_DEL)?;
        if let Some(ref color) = self.color {
            write!(f, "{}", color)?;
        }
        if let Some(ref effect) = self.effect {
            if self.color.is_some() {
                write!(f, "{}", COMPONENT_DEL)?;
            }
            write!(f, "{}", effect)?;
        }
        write!(f, "{}", COLOR_END_DEL)?;
        write!(f, "{}", self.string)?;
        write!(f, "{}", END_DEL)
    }
}
