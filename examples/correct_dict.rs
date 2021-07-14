#[macro_use]
extern crate std;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};
use word_corrector::Dictionary;

fn main() {
    let mut index = Dictionary::new();

    // Open the dictionary of words.
    let file = BufReader::new(File::open("en.txt").expect("unable to open dictionary file"));
    // Add every line to the index.
    let old_time = Instant::now();
    for line in file.lines() {
        let line = line.expect("unable to read line");
        index.add_word(line);
    }
    let new_time = Instant::now();

    eprintln!(
        "added {} words to index at {} words per second",
        index.len(),
        index.len() as f64 / (new_time - old_time).as_secs_f64(),
    );

    for quality in 1..=4 {
        // Test the recall.
        let mut correct = 0usize;
        let old_time = Instant::now();
        for ix in 0..index.len() {
            let word = index.get_word(ix).to_owned();
            let typo_word = induce_typo(&word);
            let found_word = index
                .correct(&typo_word)
                .expect("unable to correct word; empty index");
            if word == found_word {
                correct += 1;
            }
        }
        let new_time = Instant::now();

        eprintln!(
            "corrected words with recall {} at {} corrections per second with quality {}",
            correct as f64 / index.len() as f64,
            index.len() as f64 / (new_time - old_time).as_secs_f64(),
            quality
        );
    }
}

fn induce_typo(word: &str) -> String {
    let mut messed_up: String = "A".to_owned();
    messed_up.push_str(word);

    if word.len() > 4 {
        messed_up.insert(4, 'E');
    };
    messed_up
}
