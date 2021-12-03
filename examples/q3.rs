use std::{fs::File};

use std::io::{self, prelude::*, BufReader};

use std::{iter};
use std::collections::HashSet;
use std::collections::VecDeque;
use regex::{Regex};


fn main() -> io::Result<()> {
    let file = File::open("data/q3")?;
    let reader = BufReader::new(file);

    let digits = 12;
    let mut bucket: Vec<isize> = vec![0; digits];
    let mut iline = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        let num = isize::from_str_radix(line.as_str(), 2).unwrap();
        for i in 0..digits {
            bucket[i] += (num >> i) & 1;
        }
        iline += 1;
    }

    let hline = iline / 2;
    let mut gamma: isize = 0;
    let mut epsilon: isize = 0;
    for i in 0..digits {
        if bucket[i] > hline {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    dbg!(gamma, epsilon, gamma * epsilon);

    Ok(())
}