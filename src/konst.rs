use crate::{ColorDesc, ColorFmt, Effect, OneColor, OneEffect};

// Color Trait Const
pub const fn custom<D: ?Sized, const C: usize, const E: usize>(
    this: &D,
    (color, effect): ([ColorDesc; C], [Effect; E]),
) -> ColorFmt<'_, D, C, E> {
    ColorFmt {
        fmt: this,
        color,
        effect,
    }
}
pub const fn color<D: ?Sized>(this: &D, color: ColorDesc) -> OneColor<D> {
    custom(this, ([color], []))
}
pub const fn style<D: ?Sized>(this: &D, effect: Effect) -> OneEffect<D> {
    custom(this, ([], [effect]))
}

// ColorExt trait Const
pub const fn rgb<D: ?Sized>(this: &D, r: u8, g: u8, b: u8) -> OneColor<D> {
    color(this, ColorDesc::rgb(r, g, b))
}
pub const fn rgb_bg<D: ?Sized>(this: &D, r: u8, g: u8, b: u8) -> OneColor<D> {
    color(this, ColorDesc::rgb_bg(r, g, b))
}
pub const fn red<D: ?Sized>(this: &D) -> OneColor<D> {
    rgb(this, 255, 0, 0)
}
pub const fn red_bg<D: ?Sized>(this: &D) -> OneColor<D> {
    rgb_bg(this, 255, 0, 0)
}
pub const fn green<D: ?Sized>(this: &D) -> OneColor<D> {
    rgb(this, 0, 255, 0)
}
pub const fn green_bg<D: ?Sized>(this: &D) -> OneColor<D> {
    rgb_bg(this, 0, 255, 0)
}
pub const fn yellow<D: ?Sized>(this: &D) -> OneColor<D> {
    rgb(this, 255, 255, 0)
}
pub const fn yellow_bg<D: ?Sized>(this: &D) -> OneColor<D> {
    rgb_bg(this, 255, 255, 0)
}
pub const fn blue<D: ?Sized>(this: &D) -> OneColor<D> {
    rgb(this, 0, 0, 255)
}
pub const fn blue_bg<D: ?Sized>(this: &D) -> OneColor<D> {
    rgb_bg(this, 0, 0, 255)
}
pub const fn light_blue<D: ?Sized>(this: &D) -> OneColor<D> {
    rgb(this, 0, 150, 255)
}
pub const fn light_blue_bg<D: ?Sized>(this: &D) -> OneColor<D> {
    rgb_bg(this, 0, 150, 255)
}
pub const fn italic<D: ?Sized>(this: &D) -> OneEffect<D> {
    style(this, Effect::Italic)
}
pub const fn bold<D: ?Sized>(this: &D) -> OneEffect<D> {
    style(this, Effect::Bold)
}
pub const fn underline<D: ?Sized>(this: &D) -> OneEffect<D> {
    style(this, Effect::Underline)
}
pub const fn crossed_out<D: ?Sized>(this: &D) -> OneEffect<D> {
    style(this, Effect::CrossedOut)
}
