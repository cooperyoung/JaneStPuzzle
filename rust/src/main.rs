extern crate radix_trie;
extern crate colored;

mod scrabble;
mod board;

use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use scrabble::ScrabbleInfo;
use board::ScraggleBoard;
use board::is_vowel;

fn is_valid(board: &ScraggleBoard) -> bool {
    let mut letters = HashSet::new();
    for r in 0..6 {
        for c in 0..6 {
            if board.is_empty(r, c) || is_vowel(board.get(r, c)) {continue;}
            let size = letters.len();
            letters.insert(board.get(r, c));
            if size == letters.len() {return false;}
        }
    }
    true
}

fn make_chain(chain: &Vec<String>) -> Option<ScraggleBoard> {
    for r in 0..3 {
        for c in 0..3 {
            let mut board = ScraggleBoard::new();
            let mut letters = HashSet::new();
            for &c in &['B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N',
                      'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z'] {
                        letters.insert(c);
            }
            if board.make_chain(&mut chain.clone(), &mut letters, r, c, 0) { return Some(board); }
        }
    }
    None
}

fn read_quads(path: &str) -> Vec<Vec<String>> {
    let mut quads = Vec::new();

    let file = File::open(path).expect("Fuck!!");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let quad: Vec<String> = line.expect("Fuck!").split_ascii_whitespace().map(|s| s.to_string().to_uppercase()).collect();
        
        let mut unique = true;
        for i in 0..4 {
            for j in i+1..4 {
                if quad[i] == quad[j] { unique = false; }
            }
        }

        if unique {quads.push(quad);}
    }
    quads
}

fn main() {
    let info = ScrabbleInfo::new();

    let mut quads = read_quads("../quads.txt");
    quads.insert(0, vec!["EXANTHEMA".to_string(), "AXIOMATIZATIONS".to_string(), 
                         "SKYWALKS".to_string(), "SKIJORERS".to_string()]);
    quads.insert(0, vec!["JOKING".to_string(), "GROTESQUELY".to_string(), 
                         "YEAH".to_string(), "HEXYLIC".to_string()]);
    quads.insert(0, vec!["TZITZITH".to_string(), "HAPHAZARDS".to_string(), 
                         "SKYJACKS".to_string(), "SKYJACKING".to_string()]);
    quads.insert(0, vec!["TZITZIT".to_string(), "TZITZITH".to_string(), 
                         "TZITZITHS".to_string(), "TZITZITS".to_string()]);
    for mut quad in quads {
        let q = quad.clone();
        if let Some(board) = make_chain(&quad) {
            //if !is_valid(&board) { continue; }
            let score: usize = q.iter().map(|w| info.score(w)).product();
            let qwscores: Vec<_> = q.iter().map(|w| (w, info.score(w))).collect();
            if score > 500000 {
                println!("{:?}: {}\n{}", qwscores, score, board);
                println!("Old code says this {} valid", if is_valid(&board) {"is"} else {"isn't"});
            }
        }
    }
}
