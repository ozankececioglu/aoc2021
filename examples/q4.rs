use std::{fs::File};

use std::io::{self, prelude::*, BufReader};
use std::convert::TryInto;
use arrayvec::ArrayVec;

use std::{iter};
use std::collections::{HashMap, HashSet, VecDeque};
use regex::{Regex};


#[derive(Debug)]
struct Board {
    rows: [i32; 5],
    cols: [i32; 5],
}

impl Board {
    fn new() -> Board {
        Board {
            rows: [0; 5],
            cols: [0; 5],
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

    for num in numbers {
        if let Some(e) = entries.get(&num) {
            for entry in e {
                let board = &mut boards[entry.iboard];
                board.rows[entry.row] -= num;
                board.cols[entry.col] -= num;
                if board.rows[entry.row] == 0 || board.cols[entry.col] == 0 {
                    let rows = board.rows.iter().fold(0, |x, y| x + y);
                    let cols = board.cols.iter().fold(0, |x, y| x + y);
                    dbg!(rows, cols, num, rows * num);
                    return Ok(());
                }
            }
        }
    }

    return Ok(());
}