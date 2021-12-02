use std::{fs::File};

use std::io::{self, prelude::*, BufReader};

use std::{iter};
use std::collections::HashSet;
use std::collections::VecDeque;
use regex::{Regex};



fn main() -> io::Result<()> {
    let file = File::open("data/q1")?;
    let reader = BufReader::new(file);

    let mut it = reader.lines().map(|l| l.unwrap());
    let mut increase = 0;

    let mut buffer: VecDeque<i32> = VecDeque::new();
    for i in 1..=3 {
        let line = it.next().unwrap();
        buffer.push_front(line.parse::<i32>().unwrap());
    }

    for line in it {
        let val = line.parse::<i32>().unwrap();
        let prev_val = buffer.pop_back().unwrap();
        if val > prev_val {
            increase += 1;
        }
        buffer.push_front(val);
    }

    println!("!### {}", increase);

    Ok(())
}