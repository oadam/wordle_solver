use std::fmt;

use crate::color::Color;
use crate::score::Score;

pub struct Word {
    pub chars: [char; 5],
}

impl Word {
    pub fn rate_guess(&self, guess: &Word) -> Score {
        let mut result = [Color::Black; 5];
        let mut used = [false; 5];
        // assign greens
        for i in 0..5 {
            if guess.chars[i] == self.chars[i] {
                result[i] = Color::Green;
                used[i] = true;
            }
        }
        // assign yellows
        #[allow(clippy::needless_range_loop)]
        for i in 0..5 {
            if result[i] == Color::Green {
                continue;
            }
            for j in 0..5 {
                if i != j && guess.chars[i] == self.chars[j] && !used[j] {
                    used[j] = true;
                    result[i] = Color::Yellow;
                }
            }
        }
        Score::from_colors(result)
    }
    pub fn from_str(w: &str) -> Word {
        let char_vec: Vec<char> = w.chars().collect();
        let chars: [char; 5] = char_vec.try_into().expect("word does not have 5 chars");
        Word { chars }
    }
    pub fn load_words() -> Vec<Word> {
        let words_iter = include_str!("solutions.txt").lines();
        words_iter.map(Word::from_str).collect()
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.chars[0], self.chars[1], self.chars[2], self.chars[3], self.chars[4]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rating() {
        let solution = Word {
            chars: ['a', 'b', 'a', 'd', 'c'],
        };
        let guess = Word {
            chars: ['a', 'z', 'c', 'a', 'a'],
        };
        let score = Score::from_colors([
            Color::Green,
            Color::Black,
            Color::Yellow,
            Color::Yellow,
            Color::Black,
        ]);
        assert_eq!(solution.rate_guess(&guess), score);
    }
}
