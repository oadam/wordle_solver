use crate::Color;
use std::fmt;

#[derive(Debug, Hash, Copy, Clone)]
pub struct Score {
    pub code: u8,
}
impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let colors = self.to_colors();
        write!(
            f,
            "{}{}{}{}{}",
            colors[0], colors[1], colors[2], colors[3], colors[4]
        )
    }
}
impl Eq for Score {}
impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        return self.code == other.code;
    }
}
impl Score {
    pub fn to_colors(&self) -> [Color; 5] {
        let mut nn = self.code;
        let mut result = [Color::Black; 5];
        for i in 0..5 {
            result[i] = match nn % 3 {
                0 => Color::Black,
                1 => Color::Yellow,
                2 => Color::Green,
                _ => panic!("imposible modulo"),
            };
            nn = nn / 3;
        }
        result
    }
    pub fn from_colors(colors: [Color; 5]) -> Self {
        let mut result: u8 = 0;
        for i in 0..5 {
            let score = match colors[i] {
                Color::Black => 0,
                Color::Yellow => 1,
                Color::Green => 2,
            };
            result += 3_u8.pow(i.try_into().unwrap()) * score;
        }
        Score { code: result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rating() {
        let colors = [
            Color::Green,
            Color::Black,
            Color::Yellow,
            Color::Yellow,
            Color::Black,
        ];
        assert_eq!(colors, Score::from_colors(colors).to_colors());
    }
}
