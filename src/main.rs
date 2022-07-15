mod color;
mod score;
mod game;

use game::Game;
use score::Score;
use color::Color;
use std::env;
use std::fs::File;
use std::io::prelude::*;

type Word = [char; 5];

fn rate_word(solution: &Word, guess: &Word) -> Score {
    let mut result = [Color::Black; 5];
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
    Score { colors: result }
}

fn word_to_array(w: &str) -> Word {
    let char_vec: Vec<char> = w.chars().collect();
    let w2: Word = char_vec.try_into().expect("word does not have 5 chars");
    return w2;
}
fn word_to_string(w: &Word) -> String {
    return w.iter().collect();
}

fn load_words() -> Vec<Word> {
    let words_iter = include_str!("solutions.txt").lines();
    return words_iter.map(word_to_array).collect();
}

fn write_all_words(words: &Vec<String>) {
    let filename = "all_words.csv";
    let mut file = File::create(filename).unwrap();
    for (i, w) in words.iter().enumerate() {
        file.write(format!("{}\t \"{}\"\n", i, w).as_bytes())
            .unwrap();
    }
    println!("{} written !", filename);
}

fn write_all_scores() {
    let colors = [Color::Black, Color::Yellow, Color::Green];
    let filename = "all_scores.csv";
    let mut scores_file = File::create(filename).unwrap();
    for c0 in colors {
        for c1 in colors {
            for c2 in colors {
                for c3 in colors {
                    for c4 in colors {
                        let s = Score {
                            colors: [c0, c1, c2, c3, c4],
                        };
                        scores_file
                            .write(format!("{}\t{}\n", s.to_int(), s).as_bytes())
                            .unwrap();
                    }
                }
            }
        }
    }
    println!("{} written !", filename);
}

fn main() {
    if env::args().len() % 2 != 1 {
        panic!("expected pair number of arguments");
    }
    write_all_scores();
    let mut words = load_words();
    for n in 0..env::args().len() / 2 {
        let word = word_to_array(&env::args().nth(2 * n + 1).unwrap());
        let score_letters = word_to_array(&env::args().nth(2 * n + 2).unwrap());
        let score = Score {
            colors: score_letters.map(|c| match c {
                'B' => Color::Black,
                'b' => Color::Black,
                'G' => Color::Green,
                'g' => Color::Green,
                'Y' => Color::Yellow,
                'y' => Color::Yellow,
                _ => panic!("score should contain B G or Y"),
            }),
        };
        words = words
            .into_iter()
            .filter(|w| rate_word(w, &word) == score)
            .collect();
    }
    let words_str: Vec<String> = words.iter().map(word_to_string).collect();
    write_all_words(&words_str);

    let n = words.len();
    let mut all_scores: Vec<Vec<usize>> = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            all_scores[i][j] = rate_word(&words[i], &words[j]).to_int();
        }
    }
    println!("all scores computed");

    let mut game = Game::new(&all_scores, (0..n).collect());
    while !game.is_optimization_done() {
        game.refine_score();
        let (guess, score) = game.get_current_best_guess();
        println!("{}, {}", score, word_to_string(&words[guess]));
    }
    println!("{}", game.print_tree(&words_str));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rating() {
        let solution: Word = ['a', 'b', 'a', 'd', 'c'];
        let guess: Word = ['a', 'z', 'c', 'a', 'a'];
        let score: Score = Score {
            colors: [
                Color::Green,
                Color::Black,
                Color::Yellow,
                Color::Yellow,
                Color::Black,
            ],
        };
        assert_eq!(rate_word(&solution, &guess), score);
    }
}
