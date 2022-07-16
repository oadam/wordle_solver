use crate::score::Score;
use crate::word::Word;

pub struct ScoreCache {
    n: usize,
    all_scores: Vec<Score>,
}
impl ScoreCache {
    pub fn from_words(words: &Vec<Word>) -> Self {
        let n = words.len();
        let mut all_scores: Vec<Score> = Vec::with_capacity(words.len() * words.len());
        for solution in words.iter() {
            for guess in words.iter() {
                all_scores.push(solution.rate_guess(guess));
            }
        }
        ScoreCache { n, all_scores }
    }
    pub fn rate_guess(&self, solution: u16, guess: u16) -> Score {
        self.all_scores[self.n * solution as usize + guess as usize]
    }
}
