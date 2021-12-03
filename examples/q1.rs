use std::{fs::File};

use std::io::{self, prelude::*, BufReader};

use std::{iter};
use std::collections::HashSet;
use std::collections::VecDeque;
use regex::{Regex};



fn main() -> io::Result<()> {
    let file = File::open("data/q1")?;
    let reader = BufReader::new(file);

    let mut it   = reader.lines().map(|l| l?);
    let mut increase = 0;

    let mut buffer: VecDeque<i32> = VecDeque::with_capacity(3);
    for i in 1..=3 {
        let line = it.next()?;
        buffer.push_front(line.parse::<i32>()?);
    }

    for line in it {
        let val = line.parse::<i32>()?;
        let prev_val = buffer.pop_back()?;
        if val > prev_val {
            increase += 1;
        }
        buffer.push_front(val);
    }

    println!("!### {}", increase);

    Ok(())
}