extern crate radix_trie;
extern crate colored;

mod scrabble;
mod board;

use std::time::Instant;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use board::ScraggleBoard;

/*
fn search_boards(board: &mut ScraggleBoard, letters: &mut Vec<char>, info: &ScrabbleInfo,
                 count: &mut usize) {
    *count += 1;
    if *count%10000 == 0 {println!("count: {}", count);}

    if letters.is_empty() {return;}
    if letters.len() < 8 {return;}
    //if !board.is_valid(info) { println!("Invalid"); return;}
    //println!("{}", board);
    for r in (0..6).rev() {
        for c in (0..6).rev() {
            if board.is_empty(r, c) {
                let letter = letters.pop().unwrap();
                board.set(r, c, letter);
                search_boards(board, letters, info, count);
                board.unset(r, c);
                letters.push(letter);
            }
        }
    }
}
*/

fn make_chain(chain: &mut Vec<String>) -> Option<ScraggleBoard> {
    let mut board = ScraggleBoard::new();
    for r in 0..3 {
        for c in 0..3 {
            if board.make_chain(chain, r, c, 0) { return Some(board); }
        }
    }
    None
}

fn read_quads(path: String) -> Vec<Vec<String>> {
    let mut quads = Vec::new();

    let file = File::open(path).expect("Fuck!");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        quads.push(line.expect("Fuck!").split_ascii_whitespace().map(|s| s.to_string()).collect());
    }
    quads
}

fn main() {
    let now = Instant::now();
    for _ in 0..20000 {
        make_chain(&mut vec!["JOKING".to_string(), "GROTESQUELY".to_string(), 
                                     "YEAH".to_string(), "HEXYLIC".to_string()]);
    }
    println!("Took {} nanoseconds", now.elapsed().as_nanos());
    println!("Took {} milliseconds", now.elapsed().as_millis());
    println!("Took {} seconds", now.elapsed().as_secs());
}
