use std::{fs::File};

use std::io::{self, prelude::*, BufReader};

use std::{iter};
use std::collections::HashSet;
use std::collections::VecDeque;
use regex::{Regex};


fn main() -> io::Result<()> {
    let file = File::open("data/q2")?;
    let reader = BufReader::new(file);

    let mut dist = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        let mut fields = line.split_whitespace();
        let command = fields.next().unwrap();
        let arg = fields.next().unwrap().parse::<i32>().unwrap();
        match command {
            "forward" => {
                dist += arg;
                depth += aim * arg;
            },
            "down" => aim += arg,
            "up" => aim -= arg,
            _ => ()
        }
    }

    println!("!### {} {} {}", dist, depth, dist * depth);



    Ok(())
}
