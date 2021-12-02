use std::{fs::File};

use std::io::{self, prelude::*, BufReader};

use std::{iter};
use std::collections::HashSet;
use regex::{Regex};



fn main() -> io::Result<()> {
    let file = File::open("data/q1")?;
    let reader = BufReader::new(file);

    let mut it = reader.lines().map(|l| l.unwrap());
    let line = it.next().unwrap();
    let mut prev_val: i32 = line.parse::<i32>().unwrap();
    let mut iline = 0;
    let mut increase = 0;

    for line in it {
        let val = line.parse::<i32>().unwrap();
        if val > prev_val {
            increase += 1;
        }
        prev_val = val;

        iline += 1;
    }

    println!("!### {} {}", iline, increase);

    Ok(())
}