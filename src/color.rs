use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    Green,
    Yellow,
    Black,
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let txt = match self {
            Color::Green => "🟩",
            Color::Yellow => "🟨",
            Color::Black => "⬛",
        };
        write!(f, "{}", txt)
    }
}
