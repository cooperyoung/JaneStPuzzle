use std::fmt;
use std::collections::HashSet;

use colored::*;

use scrabble::ScrabbleInfo;

const MOVES: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1),
                                    ( 0, -1),          ( 0, 1),
                                    ( 1, -1), ( 1, 0), ( 1, 1)];

pub fn is_vowel(c: char) -> bool {
    c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U'
}

#[derive(Debug, Clone)]
pub struct ScraggleBoard {
    layout: [[char; 6]; 6]
}

impl fmt::Display for ScraggleBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..6 {
            for c in 0..6 {
                let cell = if ScraggleBoard::is_red(r, c) {
                    self.layout[r][c].to_string().on_red()
                } else if ScraggleBoard::is_blue(r, c) {
                    self.layout[r][c].to_string().on_blue()
                } else if ScraggleBoard::is_green(r, c) {
                    self.layout[r][c].to_string().on_green()
                } else if ScraggleBoard::is_purple(r, c) {
                    self.layout[r][c].to_string().on_bright_purple()
                } else {
                    self.layout[r][c].to_string().on_black()
                };

                write!(f, "{}", if (r+c)%2 == 0 {cell.bold()} else {cell})?;
            }
            if r < 5 {write!(f, "\n")?};
        }
        Ok(())
    }
}

#[allow(dead_code)]
impl ScraggleBoard {
    pub fn new() -> ScraggleBoard {
        ScraggleBoard { layout: [[' ', 'O', ' ', 'E', ' ', 'U'],
                                 ['I', ' ', 'A', ' ', 'A', ' '],
                                 [' ', 'E', ' ', 'I', ' ', 'O'],
                                 ['A', ' ', 'O', ' ', 'E', ' '],
                                 [' ', 'E', ' ', 'A', ' ', 'I'],
                                 ['U', ' ', 'I', ' ', 'O', ' ']]}
    }
    pub fn sample() -> ScraggleBoard {
        ScraggleBoard { layout: [['K', 'O', 'J', 'E', ' ', 'U'],
                                 ['I', 'G', 'A', ' ', 'A', ' '],
                                 ['N', 'E', 'R', 'I', 'X', 'O'],
                                 ['A', 'T', 'O', 'Y', 'E', ' '],
                                 ['S', 'E', 'L', 'A', 'H', 'I'],
                                 ['U', 'Q', 'I', 'C', 'O', ' ']]}
    }
    pub fn is_red(r: usize, c: usize) -> bool {
        r < 3 && c < 3
    }
    pub fn is_blue(r: usize, c: usize) -> bool {
        0 < r && r < 4 && 0 < c && c < 4
    }
    pub fn is_green(r: usize, c: usize) -> bool {
        1 < r && r < 5 && 1 < c && c < 5
    }
    pub fn is_purple(r: usize, c: usize) -> bool {
        2 < r && 2 < c
    }
    pub fn is_color(r: usize, c: usize, idx: usize) -> bool {
        match idx {
            0 => ScraggleBoard::is_red(r, c),
            1 => ScraggleBoard::is_blue(r, c),
            2 => ScraggleBoard::is_green(r, c),
            _ => ScraggleBoard::is_purple(r, c)
        }
    }
    pub fn is_empty(&self, r: usize, c: usize) -> bool {
        self.layout[r][c] == ' '
    }
    pub fn get(&self, r: usize, c: usize) -> char {
        self.layout[r][c]
    }
    pub fn set(&mut self, r: usize, c: usize, letter: char) {
        self.layout[r][c] = letter;
    }
    pub fn unset(&mut self, r: usize, c: usize) {
        self.layout[r][c] = ' ';
    }
    pub fn make_chain(&mut self, chain: &mut Vec<String>, letters: &mut HashSet<char>, r: usize, c: usize, idx: usize) -> bool {
        if idx > 3 { return true; }
        if chain[idx].is_empty() { return false; }

        let letter = chain[idx].chars().next().unwrap();
        // I shouldn't need this stupid line
        if self.is_empty(r, c) && !letters.contains(&letter) { return false; }

        chain[idx] = chain[idx][1..].to_string();

        if self.layout[r][c] == letter || (!is_vowel(letter) && self.is_empty(r,c)) {
            let was_empty = self.is_empty(r, c);

            if !self.is_empty(r, c) || letters.contains(&letter) {
                self.layout[r][c] = letter;
                letters.remove(&letter);

                if chain[idx].is_empty() {
                    if ScraggleBoard::is_color(r, c, idx) && ScraggleBoard::is_color(r, c, idx+1) {
                        if self.make_chain(chain, letters, r, c, idx+1) { return true; }
                    }
                } else {
                    for (dr, dc) in &MOVES {
                        let (nr, nc) = (r as isize + *dr, c as isize + *dc);
                        if nr < 0 || nc < 0 || nr > 5 || nc > 5 {continue;} 
                        let (nr, nc) = (nr as usize, nc as usize);
                        if self.make_chain(chain, letters, nr, nc, idx) {return true;}
                    }
                }
            }

            if was_empty { 
                self.layout[r][c] = ' ';
                letters.insert(letter);
            }
        }

        chain[idx] = letter.to_string() + &chain[idx];
        false
    }
    fn completion_exists(&self, r: usize, c: usize, prefix: &mut String, 
                         idx: usize, info: &ScrabbleInfo) -> bool {;
        if !info.is_prefix(&prefix) {return false;}
        if self.is_empty(r, c) {return true;}
        if ScraggleBoard::is_color(r, c, idx) && ScraggleBoard::is_color(r, c, idx+1) && info.is_word(&prefix) {return true;}
        //println!("{}", prefix);

        prefix.push(self.layout[r][c]);
        for (dr, dc) in &MOVES {
            let (nr, nc) = (r as isize + *dr, c as isize + *dc);
            if nr < 0 || nc < 0 || nr > 5 || nc > 5 {
                continue;
            } else if self.is_empty(nr as usize, nc as usize) { 
                return true;
            } else if self.completion_exists(nr as usize, nc as usize, prefix, idx, info) {
                return true;
            } 
        }
        prefix.pop();

        false
    }
    // Can this board maybe be completed?
    pub fn is_valid(&self, info: &ScrabbleInfo) -> bool {
        for i in 0..4 {
            let mut poss = false;
            for r in 0..3 {
                for c in 0..3 {
                    if self.completion_exists(r + i, c + i, &mut String::new(), i, info) {
                        poss = true;
                        break;
                    }
                }
                if poss {break;}
            }
            if !poss {return false;}
        }
        true
    }
}