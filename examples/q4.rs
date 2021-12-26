use std::{fs::File};

use std::io::{self, prelude::*, BufReader};
use std::convert::TryInto;
use arrayvec::ArrayVec;

use std::{iter};
use std::collections::{HashSet, VecDeque};
use hashbrown::HashMap;
use regex::{Regex};


#[derive(Debug)]
struct Board {
    rows: [i32; 5],
    cols: [i32; 5],
    won: bool
}

impl Board {
    fn new() -> Board {
        Board {
            rows: [0; 5],
            cols: [0; 5],
            won: false
        }
    }
}

struct Entry {
    iboard: usize,
    row: usize,
    col: usize,
}


fn main() -> io::Result<()> {
    let file = File::open("data/q4")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let numbers = lines.next().unwrap().unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut boards: Vec<Board> = Vec::new();
    let mut entries: HashMap<i32, Vec<Entry>> = HashMap::new();
    while let Some(_) = lines.next() {
        boards.push(Board::new());
        for row in 0..5 {
            let line = lines.next().unwrap().unwrap();
            let mut nums = line.split_whitespace();
            for col in 0..5 {
                let num = nums.next().unwrap().parse::<i32>().unwrap();
                let board = boards.last_mut().unwrap();
                board.rows[row] += num;
                board.cols[col] += num;
                let entry = Entry {
                    iboard: boards.len() - 1,
                    row,
                    col,
                };
                entries.entry(num).or_insert(Vec::new()).push(entry);
            }
        }
    }

    let mut boards_left = boards.len();
    for num in numbers {
        if let Some(e) = entries.get(&num) {
            for entry in e {
                let board = &mut boards[entry.iboard];
                board.rows[entry.row] -= num;
                board.cols[entry.col] -= num;
                if !board.won && (board.rows[entry.row] == 0 || board.cols[entry.col] == 0) {
                    board.won = true;
                    boards_left -= 1;
                    if boards_left == 0 {
                        let rows = board.rows.iter().sum::<i32>();
                        let cols = board.cols.iter().sum::<i32>();
                        println!("{}", rows * num);
                        return Ok(());
                    }
                }
            }
        }
    }

    return Ok(());
}