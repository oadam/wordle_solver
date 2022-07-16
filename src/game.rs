use halfbrown::HashMap;
use smallvec::{smallvec, SmallVec};
use std::collections::BinaryHeap;
use std::fmt;

use crate::{score::Score, score_cache::ScoreCache};

const NON_WINNING_SCORES_COUNT: usize = (3 as usize).pow(5) - 1;
const SMALLVEC_MAX: usize = 4;

type WordIndex = u16;
type Solution = u16;

#[derive(Debug)]
enum OptimizationStatus<'a> {
    Unstarted,
    InProgress(BinaryHeap<Guess<'a>>),
    Done(Guess<'a>),
}

pub struct LargeGame<'a> {
    score_cache: &'a ScoreCache,
    words: SmallVec<[Solution; SMALLVEC_MAX]>,
    optimization: OptimizationStatus<'a>,
}

impl fmt::Debug for LargeGame<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LargeGame")
            .field("words_len", &self.words.len())
            .field("optimization", &self.optimization)
            .finish()
    }
}

#[derive(Debug)]
pub enum Game<'a> {
    Single(Solution),
    Double(Solution, Solution),
    Large(LargeGame<'a>),
}

#[derive(Debug)]
struct Guess<'a> {
    guess: WordIndex,
    subgames: HashMap<Score, Game<'a>>,
    avg_score: f64,
    optimization_done: bool,
}

impl<'a> Ord for Guess<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other.avg_score.partial_cmp(&self.avg_score).unwrap();
    }
}
impl<'a> PartialOrd for Guess<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}
impl<'a> Eq for Guess<'a> {}
impl<'a> PartialEq for Guess<'a> {
    fn eq(&self, other: &Self) -> bool {
        return self.guess == other.guess;
    }
}

fn optimal_score(words_count: usize) -> f64 {
    match words_count {
        0 => 0.,
        1 => 1.,
        2 => 1.5,
        count => {
            // ideal guess will split words in buckets of the same size
            // and the reminder will add 1 to the size of some of the buckets
            let count_except_guess = count - 1;
            let small_buckets_size = count_except_guess / NON_WINNING_SCORES_COUNT;
            let big_buckets_size = small_buckets_size + 1;
            let big_buckets_count = count_except_guess % NON_WINNING_SCORES_COUNT;
            let small_buckets_count = NON_WINNING_SCORES_COUNT - big_buckets_count;
            let small_buckets_score = match small_buckets_size {
                0 => 0.,
                size => 1. + optimal_score(size),
            };
            let big_buckets_score = 1. + optimal_score(big_buckets_size);
            1. / (count as f64)
                * (1.
                    + (small_buckets_count as f64) * small_buckets_score
                    + (big_buckets_count as f64) * big_buckets_score)
        }
    }
}

impl<'a> Game<'a> {
    pub fn new(
        score_cache: &'a ScoreCache,
        words: SmallVec<[WordIndex; SMALLVEC_MAX]>,
    ) -> Game<'a> {
        if words.len() == 1 {
            Game::Single(words[0])
        } else if words.len() == 2 {
            Game::Double(words[0], words[1])
        } else {
            Game::Large(LargeGame {
                score_cache,
                words,
                optimization: OptimizationStatus::Unstarted,
            })
        }
    }

    pub fn get_avg_score(&self) -> f64 {
        match self {
            Game::Single(_) => 1.,
            Game::Double(_, _) => 1.5,
            Game::Large(g) => match &g.optimization {
                OptimizationStatus::Done(guess) => guess.avg_score,
                OptimizationStatus::InProgress(heap) => heap.peek().unwrap().avg_score,
                OptimizationStatus::Unstarted => optimal_score(g.words.len()),
            },
        }
    }

    pub fn print_current_best_score(&self, words: &'a Vec<String>) -> String {
        if let Game::Large(g) = self {
            if let OptimizationStatus::InProgress(ref heap) = g.optimization {
                let mut result = String::new();
                for c in heap.iter().take(3) {
                    result.push_str(&format!("{}-{} ", words[c.guess as usize], c.avg_score));
                }
                return result;
            }
        }
        String::from("optimization is over")
    }

    pub fn print_tree(&self, words: &'a Vec<String>) -> String {
        match self {
            Game::Single(w) => format!("{} !", words[*w as usize]),
            Game::Double(w1, w2) => format!("{} or {} !", words[*w1 as usize], words[*w2 as usize]),
            Game::Large(g) => {
                if let OptimizationStatus::Done(ref guess) = g.optimization {
                    let word = &words[guess.guess as usize];
                    if guess.subgames.len() == 0 {
                        word.clone() + "!"
                    } else {
                        let mut lines: Vec<String> = vec![];
                        for (score, sub) in guess.subgames.iter() {
                            for l in sub.print_tree(words).lines() {
                                lines.push(format!("{} {} {}", word, score, l));
                            }
                        }
                        lines.join("\n")
                    }
                } else {
                    panic!("optimization not done")
                }
            }
        }
    }

    pub fn is_optimization_done(&self) -> bool {
        match self {
            Game::Single(_) => true,
            Game::Double(_, _) => true,
            Game::Large(g) => {
                if let OptimizationStatus::Done(_) = g.optimization {
                    true
                } else {
                    false
                }
            }
        }
    }

    fn word_count(&self) -> usize {
        match self {
            Game::Single(_) => 1,
            Game::Double(_, _) => 2,
            Game::Large(g) => g.words.len(),
        }
    }

    pub fn refine_score(&mut self) {
        let lg = if let Game::Large(g) = self {
            Some(g)
        } else {
            None
        };
        if lg.is_none() {
            return;
        }
        let lg = lg.unwrap();
        match &mut lg.optimization {
            OptimizationStatus::Done(_) => {
                // nothing to do
            }
            OptimizationStatus::InProgress(ref mut heap) => {
                let mut guess = heap.pop().unwrap();
                let mut new_avg_score_builder = 0.;
                let mut optimization_done = true;
                for (_, game) in guess.subgames.iter_mut() {
                    game.refine_score();
                    new_avg_score_builder += game.word_count() as f64 * game.get_avg_score();
                    if !game.is_optimization_done() {
                        optimization_done = false;
                    }
                }
                guess.avg_score = 1.0 + new_avg_score_builder / lg.words.len() as f64;
                guess.optimization_done = optimization_done;
                heap.push(guess);
                let new_best = heap.peek().unwrap();
                if new_best.optimization_done {
                    lg.optimization = OptimizationStatus::Done(heap.pop().unwrap());
                }
            }
            OptimizationStatus::Unstarted => {
                let mut heap: BinaryHeap<Guess> = BinaryHeap::with_capacity(lg.words.len());
                for guess in lg.words.iter() {
                    let mut words_by_score: HashMap<Score, SmallVec<[WordIndex; SMALLVEC_MAX]>> =
                        HashMap::new();
                    for solution in lg.words.iter() {
                        // no subgame for winning game
                        if solution == guess {
                            continue;
                        }
                        let score = lg.score_cache.rate_guess(*solution, *guess);
                        if let Some(entry) = words_by_score.get_mut(&score) {
                            entry.push(*solution);
                        } else {
                            let solutions = smallvec![*solution];
                            words_by_score.insert(score, solutions);
                        }
                    }
                    let mut weighted_avg = 0.;
                    let mut subgames: HashMap<Score, Game> = HashMap::new();
                    subgames.reserve(words_by_score.len());
                    let mut optimization_done = true;
                    for (score, solutions) in words_by_score.into_iter() {
                        weighted_avg += solutions.len() as f64 * optimal_score(solutions.len());
                        let game = Game::new(lg.score_cache, solutions);
                        if !game.is_optimization_done() {
                            optimization_done = false;
                        }
                        subgames.insert(score, game);
                    }
                    heap.push(Guess {
                        guess: *guess,
                        subgames,
                        avg_score: 1.0 + weighted_avg / lg.words.len() as f64,
                        optimization_done,
                    });
                }
                lg.optimization = OptimizationStatus::InProgress(heap);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::word::Word;

    use super::*;

    macro_rules! assert_float_eq {
        ($x:expr, $y:expr) => {
            if !($x - $y < 0.00001 && $y - $x < 0.00001) {
                panic!("numbers differ too much");
            }
        };
    }

    #[test]
    fn optimal_score_test() {
        assert_float_eq!(1., optimal_score(1));
        assert_float_eq!(1.5, optimal_score(2));
        assert_float_eq!(5. / 3., optimal_score(3));
        assert_float_eq!(7. / 4., optimal_score(4));
    }

    #[test]
    fn single() {
        let all_scores = ScoreCache::from_words(&vec![Word::from_str("TOTOR")]);
        let mut game = Game::new(&all_scores, smallvec![0]);
        game.refine_score();
        assert_float_eq!(game.get_avg_score(), 1.);
    }

    #[test]
    fn double() {
        let all_scores =
            ScoreCache::from_words(&vec![Word::from_str("TOTOR"), Word::from_str("TUTUR")]);
        let mut game = Game::new(&all_scores, smallvec![0, 1]);
        game.refine_score();
        assert_float_eq!(game.get_avg_score(), 1.5);
    }

    #[test]
    fn triple() {
        let all_scores = ScoreCache::from_words(&vec![
            Word::from_str("AAAAA"),
            Word::from_str("BBBBB"),
            Word::from_str("CCCCC"),
        ]);
        let mut game = Game::new(&all_scores, smallvec![0, 1, 2]);
        game.refine_score();
        game.refine_score();
        assert_float_eq!(game.get_avg_score(), 2.0);
    }
}
