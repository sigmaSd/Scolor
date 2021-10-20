//! Simple Ansi Colors (strives for ~=0 cost)
//! ```rust,no_run
//! use scolor::ColorExt;
//! println!("{}", "hello".red().bold().underline());
//! println!("{}", "world".green().red_bg().italic());
//!
//! use scolor::{Color, Rgb, ColorType};
//! const MY_COLOR: Rgb = Rgb {r: 12, g: 100, b: 200, ctype: ColorType::Fg};
//! println!("{}", "world".rgb(MY_COLOR).bold());
//! ```
// Credits to https://stackoverflow.com/a/33206814
use std::fmt::Display;

type OneColor<'a, F> = ColorFmt<'a, F, 1, 0>;
type OneStyle<'a, F> = ColorFmt<'a, F, 0, 1>;

pub trait Color {
    type F;
    fn rgb(&self, color: Rgb) -> OneColor<Self::F>;
    fn style(&self, effect: Effect) -> OneStyle<Self::F>;
}

pub trait ColorExt {
    type F;
    fn rgb_fg(&self, r: u8, g: u8, b: u8) -> OneColor<Self::F>;
    fn rgb_bg(&self, r: u8, g: u8, b: u8) -> OneColor<Self::F>;
    fn red(&self) -> OneColor<Self::F>;
    fn red_bg(&self) -> OneColor<Self::F>;
    fn green(&self) -> OneColor<Self::F>;
    fn green_bg(&self) -> OneColor<Self::F>;
    fn yellow(&self) -> OneColor<Self::F>;
    fn yellow_bg(&self) -> OneColor<Self::F>;
    fn blue(&self) -> OneColor<Self::F>;
    fn blue_bg(&self) -> OneColor<Self::F>;
    fn light_blue(&self) -> OneColor<Self::F>;
    fn light_blue_bg(&self) -> OneColor<Self::F>;
    fn italic(&self) -> OneStyle<Self::F>;
    fn bold(&self) -> OneStyle<Self::F>;
    fn underline(&self) -> OneStyle<Self::F>;
    fn crossed_out(&self) -> OneStyle<Self::F>;
}

impl<T> Color for T
where
    T: Display,
{
    type F = T;
    fn rgb(&self, color: Rgb) -> OneColor<Self::F> {
        ColorFmt {
            fmt: self,
            color: [color],
            effect: [],
        }
    }
    fn style(&self, effect: Effect) -> OneStyle<Self::F> {
        ColorFmt {
            fmt: self,
            color: [],
            effect: [effect],
        }
    }
}

impl<T> ColorExt for T
where
    T: Color,
{
    type F = <T as Color>::F;
    fn rgb_fg(&self, r: u8, g: u8, b: u8) -> OneColor<Self::F> {
        self.rgb(Rgb {
            r,
            g,
            b,
            ctype: ColorType::Fg,
        })
    }
    fn rgb_bg(&self, r: u8, g: u8, b: u8) -> OneColor<Self::F> {
        self.rgb(Rgb {
            r,
            g,
            b,
            ctype: ColorType::Bg,
        })
    }
    fn red(&self) -> OneColor<Self::F> {
        self.rgb_fg(255, 0, 0)
    }
    fn red_bg(&self) -> OneColor<Self::F> {
        self.rgb_bg(255, 0, 0)
    }
    fn green(&self) -> OneColor<Self::F> {
        self.rgb_fg(0, 255, 0)
    }
    fn green_bg(&self) -> OneColor<Self::F> {
        self.rgb_bg(0, 255, 0)
    }
    fn yellow(&self) -> OneColor<Self::F> {
        self.rgb_fg(255, 255, 0)
    }
    fn yellow_bg(&self) -> OneColor<Self::F> {
        self.rgb_bg(255, 255, 0)
    }
    fn blue(&self) -> OneColor<Self::F> {
        self.rgb_fg(0, 0, 255)
    }
    fn blue_bg(&self) -> OneColor<Self::F> {
        self.rgb_bg(0, 0, 255)
    }
    fn light_blue(&self) -> OneColor<Self::F> {
        self.rgb_fg(0, 150, 255)
    }
    fn light_blue_bg(&self) -> OneColor<Self::F> {
        self.rgb_bg(0, 150, 255)
    }
    fn italic(&self) -> OneStyle<Self::F> {
        self.style(Effect::Italic)
    }
    fn bold(&self) -> OneStyle<Self::F> {
        self.style(Effect::Bold)
    }
    fn underline(&self) -> OneStyle<Self::F> {
        self.style(Effect::Underline)
    }
    fn crossed_out(&self) -> OneStyle<Self::F> {
        self.style(Effect::CrossedOut)
    }
}

pub struct ColorFmt<'a, D, const C: usize, const E: usize> {
    pub fmt: &'a D,
    pub color: [Rgb; C],
    pub effect: [Effect; E],
}

impl<D: Display, const C: usize, const E: usize> Display for ColorFmt<'_, D, C, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const START_DEL: &str = "\x1b[";
        const COMPONENT_DEL: &str = ";";
        const COLOR_END_DEL: &str = "m";
        const END_DEL: &str = "\x1b[0m";

        write!(f, "{}", START_DEL)?;

        for (idx, color) in self.color.iter().enumerate() {
            write!(f, "{}", color)?;
            if idx + 1 == C {
                break;
            }
            write!(f, "{}", COMPONENT_DEL)?;
        }
        for (idx, effect) in self.effect.iter().enumerate() {
            if C != 0 {
                write!(f, "{}", COMPONENT_DEL)?;
            }
            write!(f, "{}", effect)?;
            if idx + 1 == E {
                break;
            }
            write!(f, "{}", COMPONENT_DEL)?;
        }

        write!(f, "{}", COLOR_END_DEL)?;
        write!(f, "{}", self.fmt)?;
        write!(f, "{}", END_DEL)
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

pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub ctype: ColorType,
}

pub enum ColorType {
    Fg,
    Bg,
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
