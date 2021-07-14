#![no_std]

extern crate alloc;

use alloc::{borrow::ToOwned, string::String};
use hgg::HggLite;
use space::{Knn, MetricPoint, Neighbor};

pub struct Dictionary {
    hgg: HggLite<Word, ()>,
    lookup_quality: usize,
}

impl Dictionary {
    pub fn new() -> Self {
        Self {
            hgg: HggLite::new().exclude_all_searched(true),
            lookup_quality: 8,
        }
    }

    /// Default: `8`
    pub fn lookup_quality(self, lookup_quality: usize) -> Self {
        Self {
            lookup_quality,
            ..self
        }
    }

    /// Default: `64`
    pub fn insert_quality(self, insert_quality: usize) -> Self {
        Self {
            hgg: self.hgg.insert_knn(insert_quality),
            ..self
        }
    }

    pub fn add_word(&mut self, word: impl Into<String>) {
        self.hgg.insert(Word(word.into()), ());
    }

    pub fn correct(&self, word: &str) -> Option<String> {
        self.hgg
            .knn(&Word(word.to_owned()), self.lookup_quality)
            .into_iter()
            .next()
            .map(|Neighbor { index, .. }| self.hgg.get_key(index).unwrap().0.clone())
    }

    pub fn get_word(&self, word: usize) -> &str {
        &self.hgg.get_key(word).unwrap().0
    }

    pub fn len(&self) -> usize {
        self.hgg.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hgg.is_empty()
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Word(pub String);

impl MetricPoint for Word {
    type Metric = u32;

    fn distance(&self, other: &Self) -> Self::Metric {
        triple_accel::levenshtein_exp(self.0.as_bytes(), other.0.as_bytes())
    }
}
