#[derive(Clone, Copy, Debug)]
pub struct ColorDesc {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub color_type: ColorType,
}

#[derive(Clone, Copy, Debug)]
pub enum ColorType {
    Fg,
    Bg,
}

impl std::fmt::Display for ColorDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const FG_DEL: &str = "38;2";
        const BG_DEL: &str = "48;2";
        let del = match self.color_type {
            ColorType::Fg => FG_DEL,
            ColorType::Bg => BG_DEL,
        };
        write!(f, "{};{};{};{}", del, self.r, self.g, self.b)
    }
}

impl ColorDesc {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        ColorDesc {
            r,
            g,
            b,
            color_type: ColorType::Fg,
        }
    }
    pub const fn rgb_bg(r: u8, g: u8, b: u8) -> Self {
        ColorDesc {
            r,
            g,
            b,
            color_type: ColorType::Bg,
        }
    }
    pub const fn red() -> Self {
        Self::rgb(255, 0, 0)
    }
    pub const fn red_bg() -> Self {
        Self::rgb_bg(255, 0, 0)
    }
    pub const fn green() -> Self {
        Self::rgb(0, 255, 0)
    }
    pub const fn green_bg() -> Self {
        Self::rgb_bg(0, 255, 0)
    }
    pub const fn yellow() -> Self {
        Self::rgb(255, 255, 0)
    }
    pub const fn yellow_bg() -> Self {
        Self::rgb_bg(255, 255, 0)
    }
    pub const fn blue() -> Self {
        Self::rgb(0, 0, 255)
    }
    pub const fn blue_bg() -> Self {
        Self::rgb_bg(0, 0, 255)
    }
    pub const fn light_blue() -> Self {
        Self::rgb(0, 150, 255)
    }
    pub const fn light_blue_bg() -> Self {
        Self::rgb_bg(0, 150, 255)
    }
}
