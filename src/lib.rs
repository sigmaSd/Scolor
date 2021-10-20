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
    type D;
    fn rgb(&self, r: u8, g: u8, b: u8, ctype: ColorType) -> ColorFmt<Self::D>;
    fn style(&self, effect: Effect) -> ColorFmt<Self::D>;
}

pub trait ColorExt {
    type D;
    fn rgb_fg(&self, r: u8, g: u8, b: u8) -> ColorFmt<Self::D>;
    fn rgb_bg(&self, r: u8, g: u8, b: u8) -> ColorFmt<Self::D>;
    fn red(&self) -> ColorFmt<Self::D>;
    fn red_bg(&self) -> ColorFmt<Self::D>;
    fn green(&self) -> ColorFmt<Self::D>;
    fn green_bg(&self) -> ColorFmt<Self::D>;
    fn yellow(&self) -> ColorFmt<Self::D>;
    fn yellow_bg(&self) -> ColorFmt<Self::D>;
    fn blue(&self) -> ColorFmt<Self::D>;
    fn blue_bg(&self) -> ColorFmt<Self::D>;
    fn light_blue(&self) -> ColorFmt<Self::D>;
    fn light_blue_bg(&self) -> ColorFmt<Self::D>;
    fn italic(&self) -> ColorFmt<Self::D>;
    fn bold(&self) -> ColorFmt<Self::D>;
    fn underline(&self) -> ColorFmt<Self::D>;
    fn crossed_out(&self) -> ColorFmt<Self::D>;
}

impl<T> Color for T
where
    T: Display,
{
    type D = T;
    fn rgb(&self, r: u8, g: u8, b: u8, ctype: ColorType) -> ColorFmt<Self::D> {
        ColorFmt {
            string: self,
            color: Some(Rgb { r, g, b, ctype }),
            effect: None,
        }
    }
    fn style(&self, effect: Effect) -> ColorFmt<Self::D> {
        ColorFmt {
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
    type D = <T as Color>::D;
    fn rgb_fg(&self, r: u8, g: u8, b: u8) -> ColorFmt<Self::D> {
        self.rgb(r, g, b, ColorType::Fg)
    }
    fn rgb_bg(&self, r: u8, g: u8, b: u8) -> ColorFmt<Self::D> {
        self.rgb(r, g, b, ColorType::Bg)
    }
    fn red(&self) -> ColorFmt<Self::D> {
        self.rgb_fg(255, 0, 0)
    }
    fn red_bg(&self) -> ColorFmt<Self::D> {
        self.rgb_bg(255, 0, 0)
    }
    fn green(&self) -> ColorFmt<Self::D> {
        self.rgb_fg(0, 255, 0)
    }
    fn green_bg(&self) -> ColorFmt<Self::D> {
        self.rgb_bg(0, 255, 0)
    }
    fn yellow(&self) -> ColorFmt<Self::D> {
        self.rgb_fg(255, 255, 0)
    }
    fn yellow_bg(&self) -> ColorFmt<Self::D> {
        self.rgb_bg(255, 255, 0)
    }
    fn blue(&self) -> ColorFmt<Self::D> {
        self.rgb_fg(0, 0, 255)
    }
    fn blue_bg(&self) -> ColorFmt<Self::D> {
        self.rgb_bg(0, 0, 255)
    }
    fn light_blue(&self) -> ColorFmt<Self::D> {
        self.rgb_fg(0, 150, 255)
    }
    fn light_blue_bg(&self) -> ColorFmt<Self::D> {
        self.rgb_bg(0, 150, 255)
    }
    fn italic(&self) -> ColorFmt<Self::D> {
        self.style(Effect::Italic)
    }
    fn bold(&self) -> ColorFmt<Self::D> {
        self.style(Effect::Bold)
    }
    fn underline(&self) -> ColorFmt<Self::D> {
        self.style(Effect::Underline)
    }
    fn crossed_out(&self) -> ColorFmt<Self::D> {
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
pub struct ColorFmt<'a, D> {
    string: &'a D,
    color: Option<Rgb>,
    effect: Option<Effect>,
}

impl<D: Display> Display for ColorFmt<'_, D> {
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
