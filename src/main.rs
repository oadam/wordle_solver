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
struct ExploreResult<'a> {
    score: f64,
    best_guess: &'a Word,
}

fn reduce_explore_results<'a>(a: ExploreResult<'a>, b: ExploreResult<'a>) -> ExploreResult<'a> {
    if a.score < b.score {
        a
    } else {
        b
    }
}
fn rate_guess<'a>(valid_words: &Vec<&'a Word>, guess: &'a Word) -> ExploreResult<'a> {
    let mut all_scores = 0.;
    for solution in valid_words.iter() {
        if *solution == guess {
            // we found in 1
            all_scores += 1.;
            continue;
        }
        let score = rate_word(solution, guess);
        let new_valid_words: Vec<&Word> = valid_words
            .iter()
            .filter(|w| rate_word(w, guess) == score)
            .cloned()
            .collect();
        all_scores += 1. + explore(new_valid_words).score;
    }
    return ExploreResult {
        best_guess: guess,
        score: all_scores / (valid_words.len() as f64),
    };
}

fn explore<'a>(valid_words: Vec<&'a Word>) -> ExploreResult<'a> {
    if valid_words.len() == 1 {
        return ExploreResult {
            score: 1.,
            best_guess: valid_words[0],
        };
    }
    let best = valid_words
        .iter()
        .map(|guess| rate_guess(&valid_words, guess))
        .reduce(reduce_explore_results);
    return best.unwrap();
}

fn main() {
    if env::args().len() % 2 != 1 {
        panic!("expected pair number of arguments");
    }
    let mut words = load_words();
    for n in 0..env::args().len()/2 {
        let word = word_to_array(&env::args().nth(2*n + 1).unwrap());
        let score_letters = word_to_array(&env::args().nth(2*n + 2).unwrap());
        let score = score_letters.map(|c| match c {
            'B' => Color::Black,
            'b' => Color::Black,
            'G' => Color::Green,
            'g' => Color::Green,
            'Y' => Color::Yellow,
            'y' => Color::Yellow,
            _ => panic!("score should contain B G or Y")
        });
        words = words.into_iter().filter(|w| rate_word(w, &word) == score).collect();
    }
    let words_refs = words.iter().collect();
    let result = words
        .par_iter()
        .progress_count(words.len() as u64)
        .map(|guess| rate_guess(&words_refs, guess))
        .reduce_with(reduce_explore_results);
    println!("{:?}", result);
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
