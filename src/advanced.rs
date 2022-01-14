use crate::{ColorDesc, ColorFmt, Effect};

// Color trait specialization
impl<'a, D: ?Sized, const C: usize, const E: usize> ColorFmt<'a, D, C, E> {
    pub const fn custom<const NEW_C: usize, const NEW_E: usize>(
        self,
        (new_color, new_effect): ([ColorDesc; NEW_C], [Effect; NEW_E]),
    ) -> ColorFmt<'a, D, { C + NEW_C }, { E + NEW_E }> {
        let mut result_color: [ColorDesc; C + NEW_C] = [ColorDesc::rgb(0, 0, 0); { C + NEW_C }];
        {
            let mut idx = 0;
            while idx < C {
                result_color[idx] = self.color[idx];
                idx += 1;
            }
        }
        {
            let mut idx = C;
            while idx < C + NEW_C {
                result_color[idx] = new_color[idx - C];
                idx += 1;
            }
        }

        let mut result_effect: [Effect; E + NEW_E] = [Effect::CrossedOut; { E + NEW_E }];
        {
            let mut idx = 0;
            while idx < E {
                result_effect[idx] = self.effect[idx];
                idx += 1;
            }
        }
        {
            let mut idx = E;
            while idx < E + NEW_E {
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

    pub const fn color(self, color: ColorDesc) -> ColorFmt<'a, D, { C + 1 }, { E + 0 }> {
        self.custom(([color], []))
    }
    pub const fn style(self, effect: Effect) -> ColorFmt<'a, D, { C + 0 }, { E + 1 }> {
        self.custom(([], [effect]))
    }
}

// ColorExt trait specialization
impl<'a, D: ?Sized, const C: usize, const E: usize> ColorFmt<'a, D, C, E> {
    pub const fn rgb(self, r: u8, g: u8, b: u8) -> ColorFmt<'a, D, { C + 1 }, { E + 0 }> {
        self.color(ColorDesc::rgb(r, g, b))
    }
    pub const fn rgb_bg(self, r: u8, g: u8, b: u8) -> ColorFmt<'a, D, { C + 1 }, { E + 0 }> {
        self.color(ColorDesc::rgb_bg(r, g, b))
    }
    pub const fn red(self) -> ColorFmt<'a, D, { C + 1 }, { E + 0 }> {
        self.rgb(255, 0, 0)
    }
    pub const fn red_bg(self) -> ColorFmt<'a, D, { C + 1 }, { E + 0 }> {
        self.rgb_bg(255, 0, 0)
    }
    pub const fn green(self) -> ColorFmt<'a, D, { C + 1 }, { E + 0 }> {
        self.rgb(0, 255, 0)
    }
    pub const fn green_bg(self) -> ColorFmt<'a, D, { C + 1 }, { E + 0 }> {
        self.rgb_bg(0, 255, 0)
    }
    pub const fn yellow(self) -> ColorFmt<'a, D, { C + 1 }, { E + 0 }> {
        self.rgb(255, 255, 0)
    }
    pub const fn yellow_bg(self) -> ColorFmt<'a, D, { C + 1 }, { E + 0 }> {
        self.rgb_bg(255, 255, 0)
    }
    pub const fn blue(self) -> ColorFmt<'a, D, { C + 1 }, { E + 0 }> {
        self.rgb(0, 0, 255)
    }
    pub const fn blue_bg(self) -> ColorFmt<'a, D, { C + 1 }, { E + 0 }> {
        self.rgb_bg(0, 0, 255)
    }
    pub const fn light_blue(self) -> ColorFmt<'a, D, { C + 1 }, { E + 0 }> {
        self.rgb(0, 150, 255)
    }
    pub const fn light_blue_bg(self) -> ColorFmt<'a, D, { C + 1 }, { E + 0 }> {
        self.rgb_bg(0, 150, 255)
    }
    pub const fn italic(self) -> ColorFmt<'a, D, { C + 0 }, { E + 1 }> {
        self.style(Effect::Italic)
    }
    pub const fn bold(self) -> ColorFmt<'a, D, { C + 0 }, { E + 1 }> {
        self.style(Effect::Bold)
    }
    pub const fn underline(self) -> ColorFmt<'a, D, { C + 0 }, { E + 1 }> {
        self.style(Effect::Underline)
    }
    pub const fn crossed_out(self) -> ColorFmt<'a, D, { C + 0 }, { E + 1 }> {
        self.style(Effect::CrossedOut)
    }
}
