use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use radix_trie::Trie;
use radix_trie::TrieCommon;

fn letter_val(c: char) -> usize {
    if vec!['A', 'E', 'I', 'O', 'U', 'N', 'R', 'T', 'L', 'S'].contains(&c) {
        1
    } else if vec!['D', 'G'].contains(&c) {
        2
    } else if vec!['B', 'C', 'M', 'P'].contains(&c) {
        3
    } else if vec!['F', 'H', 'V', 'W', 'Y'].contains(&c) {
        4
    } else if c == 'K' {
        5
    } else if vec!['J', 'X'].contains(&c) {
        8
    } else {
        10
    }
}

pub struct ScrabbleInfo {
    lexicon: Trie<String, usize>,
}

impl ScrabbleInfo {
    pub fn new() -> ScrabbleInfo {
        let mut trie = Trie::new();

        let file = File::open("words.txt").expect("Fuck!");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(word) = line {
                let score = word.chars().map(|c| letter_val(c)).sum();
                if score >= 20 {trie.insert(word, score);}
            } else {
                panic!("Fuck!");
            }
        }

        ScrabbleInfo { lexicon: trie }
    }
    pub fn score(&self, word: &String) -> usize {
        *self.lexicon.get(word).unwrap_or(&0)
    }
    pub fn print_stats(&self) {
        println!("There are {} words in the dictionary", self.lexicon.len());
    }
    pub fn is_prefix(&self, word: &String) -> bool {
        self.lexicon.subtrie(word).is_some()
    }
    pub fn is_word(&self, word: &String) -> bool {
        self.lexicon.subtrie(word).map_or(false, |trie| trie.key().is_some())
    }
    pub fn words(&self) -> Vec<&String> {
        self.lexicon.keys().collect()
    }
}
