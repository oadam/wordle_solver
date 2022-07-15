use crate::Color;
use std::fmt;

#[derive(Debug)]
pub struct Score {
    pub colors: [Color; 5],
}
impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.colors[0], self.colors[1], self.colors[2], self.colors[3], self.colors[4]
        )
    }
}
impl Eq for Score {}
impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        return self.colors == other.colors;
    }
}
impl Score {
    pub fn to_int(&self) -> usize {
        let mut result = 0;
        for i in 0..5 {
            let score = match self.colors[i] {
                Color::Black => 0,
                Color::Yellow => 1,
                Color::Green => 2,
            };
            result += 3_usize.pow(i.try_into().unwrap()) * score;
        }
        return result;
    }
    pub fn from_int(n: usize) -> Self {
        let mut nn = n;
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
        Score { colors: result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rating() {
        let score: Score = Score {
            colors: [
                Color::Green,
                Color::Black,
                Color::Yellow,
                Color::Yellow,
                Color::Black,
            ],
        };
        assert_eq!(score, Score::from_int(score.to_int()));
    }
}
