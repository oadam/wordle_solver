use std::collections::{BinaryHeap, HashMap};
use std::fmt;

use crate::score::Score;

const NON_WINNING_SCORES_COUNT: usize = (3 as usize).pow(5) - 1;

type Word = usize;
type Solution = usize;
type ScoreInt = usize;

#[derive(Debug)]
enum OptimizationStatus<'a> {
    Unstarted,
    InProgress(BinaryHeap<Guess<'a>>),
    Done(Guess<'a>),
}

pub struct Game<'a> {
    /// solution -> guess -> score
    all_scores: &'a Vec<Vec<ScoreInt>>,
    words: Vec<Solution>,
    optimization: OptimizationStatus<'a>,
}

impl<'a> fmt::Debug for Game<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Game")
         .field("words", &self.words)
         .field("optimization", &self.optimization)
         .finish()
    }
}

#[derive(Debug)]
struct Guess<'a> {
    guess: Word,
    subgames: HashMap<ScoreInt, Game<'a>>,
    avg_score: f64,
    optimization_done: bool,
}

impl<'a> Ord for Guess<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.avg_score.partial_cmp(&other.avg_score).unwrap();
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
    pub fn new(all_scores: &'a Vec<Vec<ScoreInt>>, words: Vec<usize>) -> Game<'a> {
        let optimization = if words.len() == 1 {
            OptimizationStatus::Done(Guess {
                guess: words[0],
                subgames: HashMap::new(),
                avg_score: 1.,
                optimization_done: true,
            })
        } else if words.len() == 2 {
            // optimize len 2, since there is no best strategy than playing the first word
            let mut subgames: HashMap<ScoreInt, Game> = HashMap::new();
            let score_if_first_guess_wrong = all_scores[words[1]][words[0]];
            let game_if_first_guess_wrong = Game::new(all_scores, vec![words[1]]);
            subgames.insert(score_if_first_guess_wrong, game_if_first_guess_wrong);
            OptimizationStatus::Done(Guess {
                guess: words[0],
                subgames: subgames,
                avg_score: 1.5,
                optimization_done: true,
            })
        } else {
            OptimizationStatus::Unstarted
        };
        return Game {
            all_scores,
            words,
            optimization,
        };
    }

    fn get_best_guess(&self) -> Option<&Guess> {
        match &self.optimization {
            OptimizationStatus::Done(guess) => Some(guess),
            OptimizationStatus::InProgress(heap) => Some(heap.peek().unwrap()),
            OptimizationStatus::Unstarted => None,
        }
    }
    pub fn get_avg_score(&self) -> f64 {
        if let Some(guess) = self.get_best_guess() {
            guess.avg_score
        } else {
            optimal_score(self.words.len())

        }
    }

    pub fn get_current_best_guess(&self) -> (Word, f64) {
        let guess = self.get_best_guess().unwrap();
        (guess.guess, guess.avg_score)
    }

    pub fn print_tree(&self, words: &'a Vec<String>) -> String {
        if let OptimizationStatus::Done(ref guess) = self.optimization {
            let mut lines: Vec<String> = vec![];
            let word  = &words[guess.guess];
            if guess.subgames.len() == 0 {
                return word.clone() + "!";
            }
            for (score, sub) in guess.subgames.iter() {
                let score_str = Score::from_int(*score).to_string();
                for l in sub.print_tree(words).lines() {
                    lines.push(format!("{} {} {}", word, score_str, l));
                }
            }
            lines.join("\n")
        } else {
            panic!("optimization not done")
        }
    }

    pub fn is_optimization_done(&self) -> bool {
        if let OptimizationStatus::Done(_) = self.optimization {
            true
        } else {
            false
        }
    }

    pub fn refine_score(&mut self) {
        match &mut self.optimization {
            OptimizationStatus::Done(_) => {
                // nothing to do
            }
            OptimizationStatus::InProgress(ref mut heap) => {
                let mut guess = heap.pop().unwrap();
                let mut new_avg_score_builder = 0.;
                if self.words.contains(&guess.guess) {
                    new_avg_score_builder += 1.0;
                }
                let mut optimization_done = true;
                for (_, game) in guess.subgames.iter_mut() {
                    game.refine_score();
                    new_avg_score_builder += game.words.len() as f64 * (1.0 + game.get_avg_score());
                    if let OptimizationStatus::InProgress(_) = game.optimization {
                        optimization_done = false;
                    }
                }
                guess.avg_score = new_avg_score_builder / self.words.len() as f64;
                guess.optimization_done = optimization_done;
                heap.push(guess);
                let new_best = heap.peek().unwrap();
                if new_best.optimization_done {
                    self.optimization = OptimizationStatus::Done(heap.pop().unwrap());
                }
            }
            OptimizationStatus::Unstarted => {
                let mut heap: BinaryHeap<Guess> = BinaryHeap::with_capacity(self.words.len());
                for guess in self.words.iter() {
                    let mut words_by_score: HashMap<ScoreInt, Vec<Word>> = HashMap::new();
                    for solution in self.words.iter() {
                        // no subgame for winning game
                        if solution == guess {
                            continue;
                        }
                        let score = self.all_scores[*solution][*guess];
                        if let Some(entry) = words_by_score.get_mut(&score) {
                            entry.push(*solution);
                        } else {
                            words_by_score.insert(score, vec![*solution]);
                        }
                    }
                    let mut weighted_avg = 0.;
                    let mut subgames: HashMap<ScoreInt, Game> = HashMap::new();
                    let mut optimization_done = true;
                    for (score, solutions) in words_by_score.into_iter() {
                        weighted_avg += solutions.len() as f64 * optimal_score(solutions.len());
                        let game = Game::new(self.all_scores, solutions);
                        match game.optimization {
                            OptimizationStatus::InProgress(_) => {
                                optimization_done = false;
                            }
                            OptimizationStatus::Unstarted => {
                                optimization_done = false;
                            }
                            OptimizationStatus::Done(_) => { //nothing
                            }
                        }
                        subgames.insert(score, game);
                    }
                    heap.push(Guess {
                        guess: *guess,
                        subgames,
                        avg_score: weighted_avg / self.words.len() as f64,
                        optimization_done,
                    });
                }
                self.optimization = OptimizationStatus::InProgress(heap);
            }
        }
    }
}

#[cfg(test)]
mod tests {
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
        let all_scores = vec![vec![42]];
        let mut game = Game::new(&all_scores, vec![0]);
        game.refine_score();
        assert_float_eq!(game.get_avg_score(), 1.);
    }

    #[test]
    fn double() {
        let all_scores = vec![vec![42, 43], vec![42, 43]];
        let mut game = Game::new(&all_scores, vec![0, 1]);
        game.refine_score();
        assert_float_eq!(game.get_avg_score(), 1.5);
    }

    #[test]
    fn triple() {
        // TOTOR, TUTUR, MAMAM
        let words = vec![0, 1, 2];
        let all_scores = vec![vec![42, 43, 45], vec![42, 43, 45], vec![42, 43, 45]];
        let mut game = Game::new(&all_scores, words);
        game.refine_score();
        game.refine_score();
        assert_float_eq!(game.get_avg_score(), 2.0);
    }
}
