use std::fmt::Display;

use crate::{ColorDesc, ColorFmt, ColorType, Effect};

// Color trait specialization
impl<'a, D: Display, const C: usize, const E: usize> ColorFmt<'a, D, C, E> {
    pub fn color<const CC: usize>(self, color: ColorDesc) -> ColorFmt<'a, D, CC, E> {
        assert_eq!(CC, C + 1);

        let mut new_color: [ColorDesc; CC] = [ColorDesc {
            r: 0,
            g: 0,
            b: 0,
            color_type: ColorType::Fg,
        }; CC];
        for (idx, c) in self.color.into_iter().enumerate() {
            new_color[idx] = c;
        }
        new_color[C] = color;

        ColorFmt {
            fmt: self.fmt,
            color: new_color,
            effect: self.effect,
        }
    }
    pub fn style<const EE: usize>(self, effect: Effect) -> ColorFmt<'a, D, C, EE> {
        assert_eq!(EE, E + 1);

        let mut new_effect: [Effect; EE] = [Effect::CrossedOut; EE];
        for (idx, e) in self.effect.into_iter().enumerate() {
            new_effect[idx] = e;
        }
        new_effect[E] = effect;

        ColorFmt {
            fmt: self.fmt,
            color: self.color,
            effect: new_effect,
        }
    }
}

// ColorExt trait specialization
impl<'a, D: Display, const C: usize, const E: usize> ColorFmt<'a, D, C, E> {
    pub fn rgb<const CC: usize>(self, r: u8, g: u8, b: u8) -> ColorFmt<'a, D, CC, E> {
        self.color(ColorDesc {
            r,
            g,
            b,
            color_type: ColorType::Fg,
        })
    }
    pub fn rgb_bg<const CC: usize>(self, r: u8, g: u8, b: u8) -> ColorFmt<'a, D, CC, E> {
        self.color(ColorDesc {
            r,
            g,
            b,
            color_type: ColorType::Bg,
        })
    }
    pub fn red<const CC: usize>(self) -> ColorFmt<'a, D, CC, E> {
        self.rgb(255, 0, 0)
    }
    pub fn red_bg<const CC: usize>(self) -> ColorFmt<'a, D, CC, E> {
        self.rgb_bg(255, 0, 0)
    }
    pub fn green<const CC: usize>(self) -> ColorFmt<'a, D, CC, E> {
        self.rgb(0, 255, 0)
    }
    pub fn green_bg<const CC: usize>(self) -> ColorFmt<'a, D, CC, E> {
        self.rgb_bg(0, 255, 0)
    }
    pub fn yellow<const CC: usize>(self) -> ColorFmt<'a, D, CC, E> {
        self.rgb(255, 255, 0)
    }
    pub fn yellow_bg<const CC: usize>(self) -> ColorFmt<'a, D, CC, E> {
        self.rgb_bg(255, 255, 0)
    }
    pub fn blue<const CC: usize>(self) -> ColorFmt<'a, D, CC, E> {
        self.rgb(0, 0, 255)
    }
    pub fn blue_bg<const CC: usize>(self) -> ColorFmt<'a, D, CC, E> {
        self.rgb_bg(0, 0, 255)
    }
    pub fn light_blue<const CC: usize>(self) -> ColorFmt<'a, D, CC, E> {
        self.rgb(0, 150, 255)
    }
    pub fn light_blue_bg<const CC: usize>(self) -> ColorFmt<'a, D, CC, E> {
        self.rgb_bg(0, 150, 255)
    }
    pub fn italic<const EE: usize>(self) -> ColorFmt<'a, D, C, EE> {
        self.style(Effect::Italic)
    }
    pub fn bold<const EE: usize>(self) -> ColorFmt<'a, D, C, EE> {
        self.style(Effect::Bold)
    }
    pub fn underline<const EE: usize>(self) -> ColorFmt<'a, D, C, EE> {
        self.style(Effect::Underline)
    }
    pub fn crossed_out<const EE: usize>(self) -> ColorFmt<'a, D, C, EE> {
        self.style(Effect::CrossedOut)
    }
}
