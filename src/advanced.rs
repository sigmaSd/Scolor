use crate::{ColorDesc, ColorFmt, Effect};

// Color trait specialization
impl<'a, D: ?Sized, const C: usize, const E: usize> ColorFmt<'a, D, C, E> {
    pub const fn custom<
        const RESULT_C: usize,
        const RESULT_E: usize,
        const NEW_C: usize,
        const NEW_E: usize,
    >(
        self,
        (new_color, new_effect): ([ColorDesc; NEW_C], [Effect; NEW_E]),
    ) -> ColorFmt<'a, D, RESULT_C, RESULT_E> {
        if RESULT_C != C + NEW_C {
            panic!("Generic must be equal to the total number of colors.")
        }
        if RESULT_E != E + NEW_E {
            panic!("Generic must be equal to the total number of styles.")
        }

        let mut result_color: [ColorDesc; RESULT_C] = [ColorDesc::rgb(0, 0, 0); RESULT_C];
        {
            let mut idx = 0;
            while idx < C {
                result_color[idx] = self.color[idx];
                idx += 1;
            }
        }
        {
            let mut idx = C;
            while idx < RESULT_C {
                result_color[idx] = new_color[idx - C];
                idx += 1;
            }
        }

        let mut result_effect: [Effect; RESULT_E] = [Effect::CrossedOut; RESULT_E];
        {
            let mut idx = 0;
            while idx < E {
                result_effect[idx] = self.effect[idx];
                idx += 1;
            }
        }
        {
            let mut idx = E;
            while idx < RESULT_E {
                result_effect[idx] = new_effect[idx - E];
                idx += 1;
            }
        }

        ColorFmt {
            fmt: self.fmt,
            color: result_color,
            effect: result_effect,
        }
    }

    pub const fn color<const CC: usize>(self, color: ColorDesc) -> ColorFmt<'a, D, CC, E> {
        self.custom(([color], []))
    }
    pub const fn style<const EE: usize>(self, effect: Effect) -> ColorFmt<'a, D, C, EE> {
        self.custom(([], [effect]))
    }
}

// ColorExt trait specialization
impl<'a, D: ?Sized, const C: usize, const E: usize> ColorFmt<'a, D, C, E> {
    pub const fn rgb<const CC: usize>(self, r: u8, g: u8, b: u8) -> ColorFmt<'a, D, CC, E> {
        self.color(ColorDesc::rgb(r, g, b))
    }
    pub const fn rgb_bg<const CC: usize>(self, r: u8, g: u8, b: u8) -> ColorFmt<'a, D, CC, E> {
        self.color(ColorDesc::rgb_bg(r, g, b))
    }
    pub const fn red<const CC: usize>(self) -> ColorFmt<'a, D, CC, E> {
        self.rgb(255, 0, 0)
    }
    pub const fn red_bg<const CC: usize>(self) -> ColorFmt<'a, D, CC, E> {
        self.rgb_bg(255, 0, 0)
    }
    pub const fn green<const CC: usize>(self) -> ColorFmt<'a, D, CC, E> {
        self.rgb(0, 255, 0)
    }
    pub const fn green_bg<const CC: usize>(self) -> ColorFmt<'a, D, CC, E> {
        self.rgb_bg(0, 255, 0)
    }
    pub const fn yellow<const CC: usize>(self) -> ColorFmt<'a, D, CC, E> {
        self.rgb(255, 255, 0)
    }
    pub const fn yellow_bg<const CC: usize>(self) -> ColorFmt<'a, D, CC, E> {
        self.rgb_bg(255, 255, 0)
    }
    pub const fn blue<const CC: usize>(self) -> ColorFmt<'a, D, CC, E> {
        self.rgb(0, 0, 255)
    }
    pub const fn blue_bg<const CC: usize>(self) -> ColorFmt<'a, D, CC, E> {
        self.rgb_bg(0, 0, 255)
    }
    pub const fn light_blue<const CC: usize>(self) -> ColorFmt<'a, D, CC, E> {
        self.rgb(0, 150, 255)
    }
    pub const fn light_blue_bg<const CC: usize>(self) -> ColorFmt<'a, D, CC, E> {
        self.rgb_bg(0, 150, 255)
    }
    pub const fn italic<const EE: usize>(self) -> ColorFmt<'a, D, C, EE> {
        self.style(Effect::Italic)
    }
    pub const fn bold<const EE: usize>(self) -> ColorFmt<'a, D, C, EE> {
        self.style(Effect::Bold)
    }
    pub const fn underline<const EE: usize>(self) -> ColorFmt<'a, D, C, EE> {
        self.style(Effect::Underline)
    }
    pub const fn crossed_out<const EE: usize>(self) -> ColorFmt<'a, D, C, EE> {
        self.style(Effect::CrossedOut)
    }
}
