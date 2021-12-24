#[derive(Clone, Copy, Debug)]
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
