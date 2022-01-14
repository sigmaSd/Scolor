//! Simple Ansi Colors (strives for ~=0 cost)
//! ```rust
//! use scolor::ColorExt;
//! # #[cfg(not(feature="zero-cost"))]
//! # {
//! println!("{}", "hello".red().bold().underline());
//! println!("{}", "world".green().red_bg().italic());
//!
//! use scolor::{Color, CustomStyle, ColorDesc, Effect};
//! const LIGHT_BLUE_ITALIC_BOLD: CustomStyle<2, 2> =
//! ([ColorDesc::light_blue(), ColorDesc::red_bg()], [Effect::Italic, Effect::Bold]);
//!
//! println!("{}", "world".custom(LIGHT_BLUE_ITALIC_BOLD));
//! # }
//! ```
//!
//! Const equivalent of trait functions are provided as freestanding top-level functions
//! ```rust
//! const BLUE_WORLD: scolor::ColorFmt<'_,str,1,0> = scolor::blue("world");
//! ```
//!
//! For even more zero cost power you can enable `zero-cost` feature
//!
//! It makes the generated ASCII code as optimal as it can be
//!
//! But the cost is that it's less ergonomic, the API is invoked like this:
//! ```rust
//! use scolor::ColorExt;
//! # #[cfg(feature="zero-cost")]
//! println!("{}", "hello".green().bold::<1>().red_bg::<2>().italic::<2>());
//!
//! use scolor::{ColorDesc, ColorFmt, Effect, green};
//! # #[cfg(feature="zero-cost")]
//! const _:() = {
//!     let fmt = green("hello").italic::<1>().bold::<2>().red_bg::<2>().crossed_out::<3>();
//!     assert!(matches!(ColorFmt{fmt:"hello",color:[ColorDesc::green(),ColorDesc::red_bg()],effect:[Effect::Italic, Effect::Bold]}, fmt));
//! };

//! ```
//!
//! For an even better API you can use the nightly branch <https://github.com/sigmaSd/Scolor/tree/nightly>
// Credits to https://stackoverflow.com/a/33206814
use std::fmt::Display;

#[cfg(feature = "zero-cost")]
mod advanced;

mod color_desc;
pub use color_desc::*;
mod effect;
pub use effect::*;
mod konst;
pub use konst::*;

pub type CustomStyle<const C: usize, const E: usize> = ([ColorDesc; C], [Effect; E]);

type OneColor<'a, F> = ColorFmt<'a, F, 1, 0>;
type OneEffect<'a, F> = ColorFmt<'a, F, 0, 1>;

pub trait Color {
    fn custom<const C: usize, const E: usize>(
        &self,
        color_and_effect: CustomStyle<C, E>,
    ) -> ColorFmt<'_, Self, C, E> {
        ColorFmt {
            fmt: self,
            color: color_and_effect.0,
            effect: color_and_effect.1,
        }
    }
    fn color(&self, color: ColorDesc) -> OneColor<Self> {
        self.custom(([color], []))
    }
    fn style(&self, effect: Effect) -> OneEffect<Self> {
        self.custom(([], [effect]))
    }
}

pub trait ColorExt
where
    Self: Color,
{
    fn rgb(&self, r: u8, g: u8, b: u8) -> OneColor<Self> {
        self.color(ColorDesc::rgb(r, g, b))
    }
    fn rgb_bg(&self, r: u8, g: u8, b: u8) -> OneColor<Self> {
        self.color(ColorDesc::rgb_bg(r, g, b))
    }
    fn red(&self) -> OneColor<Self> {
        self.rgb(255, 0, 0)
    }
    fn red_bg(&self) -> OneColor<Self> {
        self.rgb_bg(255, 0, 0)
    }
    fn green(&self) -> OneColor<Self> {
        self.rgb(0, 255, 0)
    }
    fn green_bg(&self) -> OneColor<Self> {
        self.rgb_bg(0, 255, 0)
    }
    fn yellow(&self) -> OneColor<Self> {
        self.rgb(255, 255, 0)
    }
    fn yellow_bg(&self) -> OneColor<Self> {
        self.rgb_bg(255, 255, 0)
    }
    fn blue(&self) -> OneColor<Self> {
        self.rgb(0, 0, 255)
    }
    fn blue_bg(&self) -> OneColor<Self> {
        self.rgb_bg(0, 0, 255)
    }
    fn light_blue(&self) -> OneColor<Self> {
        self.rgb(0, 150, 255)
    }
    fn light_blue_bg(&self) -> OneColor<Self> {
        self.rgb_bg(0, 150, 255)
    }
    fn italic(&self) -> OneEffect<Self> {
        self.style(Effect::Italic)
    }
    fn bold(&self) -> OneEffect<Self> {
        self.style(Effect::Bold)
    }
    fn underline(&self) -> OneEffect<Self> {
        self.style(Effect::Underline)
    }
    fn crossed_out(&self) -> OneEffect<Self> {
        self.style(Effect::CrossedOut)
    }
}

impl<T: Display> Color for T {}
impl<T: Display> ColorExt for T {}

#[derive(Debug)]
pub struct ColorFmt<'a, D: ?Sized, const C: usize, const E: usize> {
    pub fmt: &'a D,
    pub color: [ColorDesc; C],
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
            if idx == 0 && C != 0 {
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
