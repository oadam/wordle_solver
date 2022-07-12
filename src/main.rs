use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::env;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Color {
    Green,
    Yellow,
    Black,
}

type Word = [char; 5];
type Score = [Color; 5];

fn rate_word(solution: &Word, guess: &Word) -> Score {
    let mut result: Score = [Color::Black; 5];
    let mut used = [false; 5];
    // assign greens
    for i in 0..5 {
        if guess[i] == solution[i] {
            result[i] = Color::Green;
            used[i] = true;
        }
    }
    // assign yellows
    for i in 0..5 {
        if result[i] == Color::Green {
            continue;
        }
        for j in 0..5 {
            if i != j && guess[i] == solution[j] && !used[j] {
                used[j] = true;
                result[i] = Color::Yellow;
            }
        }
    }
    return result;
}

fn score_to_int(s: &Score) -> usize {
    let mut result = 0;
    for i in 0..5 {
        let score = match s[i] {
            Color::Black => 0,
            Color::Yellow => 1,
            Color::Green => 2,
        };
        result += 3_usize.pow(i.try_into().unwrap()) * score;
    }
    return result;
}

fn word_to_array(w: &str) -> Word {
    let char_vec: Vec<char> = w.chars().collect();
    let w2: Word = char_vec.try_into().expect("word does not have 5 chars");
    return w2;
}

fn load_words() -> Vec<Word> {
    let words_iter = include_str!("solutions.txt").lines();
    return words_iter.map(word_to_array).collect();
}

#[derive(Copy, Clone, Debug)]
struct ExploreResult {
    score: f32,
    best_guess: usize,
}

fn reduce_explore_results(a: ExploreResult, b: ExploreResult) -> ExploreResult {
    if a.score < b.score {
        a
    } else {
        b
    }
}

fn rate_guess(
    pool: &Pool<Vec<usize>>,
    n: usize,
    all_scores: &Vec<Vec<usize>>,
    remaining_words: &Vec<usize>,
    guess: usize,
) -> ExploreResult {
    let mut score_sum = 0.;
    for solution in remaining_words {
        if *solution == guess {
            // we found in 1
            score_sum += 1.;
            continue;
        }
        let score = all_scores[*solution][guess];
        let mut new_valid_words: Vec<usize> = Vec::with_capacity(50);
        for w in remaining_words {
            if all_scores[*w][guess] == score {
                new_valid_words.push(*w);
            }
        }
        score_sum += 1. + explore(pool, n, all_scores, new_valid_words).score;
    }
    return ExploreResult {
        best_guess: guess,
        score: score_sum / (remaining_words.len() as f32),
    };
}

fn explore(
    pool: &Pool<Vec<usize>>,
    n: usize,
    all_scores: &Vec<Vec<usize>>,
    remaining_words: Vec<usize>,
) -> ExploreResult {
    if remaining_words.len() == 1 {
        return ExploreResult {
            score: 1.,
            best_guess: remaining_words[0],
        };
    }
    let best = remaining_words
        .iter()
        .map(|guess| rate_guess(pool, n, all_scores, &remaining_words, *guess))
        .reduce(reduce_explore_results);
    return best.unwrap();
}

fn main() {
    if env::args().len() % 2 != 1 {
        panic!("expected pair number of arguments");
    }
    let mut words = load_words();
    for n in 0..env::args().len() / 2 {
        let word = word_to_array(&env::args().nth(2 * n + 1).unwrap());
        let score_letters = word_to_array(&env::args().nth(2 * n + 2).unwrap());
        let score = score_letters.map(|c| match c {
            'B' => Color::Black,
            'b' => Color::Black,
            'G' => Color::Green,
            'g' => Color::Green,
            'Y' => Color::Yellow,
            'y' => Color::Yellow,
            _ => panic!("score should contain B G or Y"),
        });
        words = words
            .into_iter()
            .filter(|w| rate_word(w, &word) == score)
            .collect();
    }
    let n = words.len();
    let mut all_scores: Vec<Vec<usize>> = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            all_scores[i][j] = score_to_int(&rate_word(&words[i], &words[j]));
        }
    }

    let pool: Pool<Vec<usize>> = Pool::new(2, || Vec::new());

    let remaining_words: Vec<usize> = (0..n).collect();
    let result = (0..n)
        .into_par_iter()
        .progress_count(n.try_into().unwrap())
        .map(|i| rate_guess(&pool, n, &all_scores, &remaining_words, i))
        .reduce_with(reduce_explore_results)
        .unwrap();

    println!(
        "best word is {:?} for an avg score of {}",
        words[result.best_guess], result.score
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rating() {
        let solution: Word = ['a', 'b', 'a', 'd', 'c'];
        let guess: Word = ['a', 'z', 'c', 'a', 'a'];
        let score: Score = [
            Color::Green,
            Color::Black,
            Color::Yellow,
            Color::Yellow,
            Color::Black,
        ];
        assert!(rate_word(&solution, &guess).iter().eq(score.iter()));
    }
}
