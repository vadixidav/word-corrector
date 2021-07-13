#[macro_use]
extern crate std;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};
use word_corrector::Index;

fn main() {
    let mut index = Index::new();

    // Open the dictionary of words.
    let file = BufReader::new(File::open("en.txt").expect("unable to open dictionary file"));
    // Add every line to the index.
    let old_time = Instant::now();
    for (ix, line) in file.lines().enumerate() {
        let line = line.expect("unable to read line");
        index.add_word(line);
        if ix % 100 == 0 {
            eprintln!("proccessed index {}", ix);
        }
    }
    let new_time = Instant::now();

    eprintln!(
        "added {} lines at {} lines per second",
        index.hgg.len(),
        index.hgg.len() as f64 / (new_time - old_time).as_secs_f64()
    );

    // Test the recall.
    let old_time = Instant::now();
    let mut correct = 0usize;
    for ix in 0..index.hgg.len() {
        let word = index.hgg.get_key(ix).unwrap().0.clone();
        let typo_word = induce_typo(&word);
        let found_word = index
            .correct(&typo_word, 16)
            .expect("unable to correct word; empty index");
        if word == found_word {
            correct += 1;
        }
        if ix % 1000 == 0 {
            eprintln!("processed index {}", ix);
        }
    }
    let new_time = Instant::now();

    eprintln!(
        "recall {} at {} corrections per second",
        correct as f64 / index.hgg.len() as f64,
        index.hgg.len() as f64 / (new_time - old_time).as_secs_f64()
    );
}

fn induce_typo(word: &str) -> String {
    let mut messed_up: String = "A".to_owned();
    messed_up.push_str(word);

    if word.len() > 4 {
        messed_up.insert(4, 'E');
    };
    messed_up
}
