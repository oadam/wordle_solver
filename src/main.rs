mod color;
mod game;
mod score;
mod score_cache;
mod word;

use color::Color;
use game::Game;
use score::Score;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use word::Word;

use crate::score_cache::ScoreCache;

fn write_all_words(words: &Vec<Word>) {
    let filename = "all_words.csv";
    let mut file = File::create(filename).unwrap();
    for (i, w) in words.iter().enumerate() {
        file.write(format!("{}\t \"{}\"\n", i, w).as_bytes())
            .unwrap();
    }
    println!("{} written !", filename);
}

fn write_all_scores() {
    let filename = "all_scores.csv";
    let mut scores_file = File::create(filename).unwrap();
    for code in 0..3_usize.pow(5) {
        let score = Score { code: code as u8 };
        scores_file
            .write(format!("{}\t{}\n", code, score).as_bytes())
            .unwrap();
    }
    println!("{} written !", filename);
}

fn main() {
    if env::args().len() % 2 != 1 {
        panic!("expected pair number of arguments");
    }
    write_all_scores();
    let mut words = Word::load_words();
    for n in 0..env::args().len() / 2 {
        let word = Word::from_str(&env::args().nth(2 * n + 1).unwrap());
        let score_letters = Word::from_str(&env::args().nth(2 * n + 2).unwrap());
        let score_colors = score_letters.chars.map(|c| match c {
            'B' => Color::Black,
            'b' => Color::Black,
            'G' => Color::Green,
            'g' => Color::Green,
            'Y' => Color::Yellow,
            'y' => Color::Yellow,
            _ => panic!("score should contain B G or Y"),
        });
        let score = Score::from_colors(score_colors);
        words = words
            .into_iter()
            .filter(|w| w.rate_guess(&word) == score)
            .collect();
    }
    write_all_words(&words);

    let n = words.len();
    let score_cache = ScoreCache::from_words(&words);
    println!("all scores computed");

    let words_str = words.iter().map(|w| w.to_string()).collect();
    let mut game = Game::new(&score_cache, (0..n as u16).collect());
    while !game.is_optimization_done() {
        game.refine_score();
        println!("{}", game.print_current_best_score(&words_str));
    }
    println!("{}", game.print_tree(&words_str));
    println!("average score : {}", game.get_avg_score());
}
