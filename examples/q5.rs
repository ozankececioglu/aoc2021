use std::{fs::File};

use std::io::{self, prelude::*, BufReader};
use std::convert::TryInto;
use arrayvec::ArrayVec;

use std::{iter};
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::swap;
use itertools::Itertools;
use regex::{Regex};


fn main() -> io::Result<()> {
    let file = File::open("data/q5")?;
    let reader = BufReader::new(file);
    let mut vals: HashMap<(i32, i32), i32> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let (mut x0, mut y0, mut x1, mut y1) = line.split(" -> ")
            .map(|f| f.split(",").map(|i| i.parse::<i32>().unwrap()))
            .flatten().collect_tuple::<(i32, i32, i32, i32)>().unwrap();
        if x0 == x1 {
            if y0 > y1 {
                swap(&mut y0, &mut y1);
            }
            for y in y0..=y1 {
                *vals.entry((x0, y)).or_insert(0) += 1;
            }
        } else if y0 == y1 {
            if x0 > x1 {
                swap(&mut x0, &mut x1);
            }
            for x in x0..=x1 {
                *vals.entry((x, y0)).or_insert(0) += 1;
            }
        } else if i32::abs(y1 - y0) == i32::abs(x1 - x0) {
            let xc = if x1 > x0 {1} else {-1};
            let yc = if y1 > y0 {1} else {-1};
            for d in 0..=i32::abs(x1 - x0) {
                *vals.entry((x0 + d * xc, y0 + d * yc)).or_insert(0) += 1;
            }
        }
    }

    // dbg!(&vals);
    dbg!(vals.values().filter(|x| **x > 1).count());

    Ok(())
}