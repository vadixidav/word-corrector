#![no_std]

extern crate alloc;

use alloc::{borrow::ToOwned, string::String};
use edit_distance::edit_distance;
use hgg::HggLite;
use space::{Knn, MetricPoint, Neighbor};

pub struct Index {
    hgg: HggLite<Word, ()>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            hgg: HggLite::new().exclude_all_searched(true),
        }
    }

    pub fn add_word(&mut self, word: impl Into<String>) {
        self.hgg.insert(Word(word.into()), ());
    }

    pub fn correct(&self, word: &str, quality: usize) -> Option<String> {
        self.hgg
            .knn(&Word(word.to_owned()), quality)
            .into_iter()
            .next()
            .map(|Neighbor { index, .. }| self.hgg.get_key(index).unwrap().0.clone())
    }
}

impl Default for Index {
    fn default() -> Self {
        Self::new()
    }
}

struct Word(String);

impl MetricPoint for Word {
    type Metric = u32;

    fn distance(&self, other: &Self) -> Self::Metric {
        edit_distance(&self.0, &other.0) as u32
    }
}
