use std::{fs::File};

use std::io::{self, prelude::*, BufReader};

use std::{iter};
use std::collections::HashSet;
use std::collections::VecDeque;
use regex::{Regex};


fn main() -> io::Result<()> {
    let file = File::open("data/q3")?;
    let reader = BufReader::new(file);

    let digits: u32 = 11;
    let mut oxy: Vec<u32> = Vec::new();

    for line in reader.lines().map(|l| l.unwrap()) {
        let num = u32::from_str_radix(line.as_str(), 2).unwrap();
        oxy.push(num);
    }

    let mut co2: Vec<u32> = oxy.clone();

    for i in 0..=digits {
        let c = oxy.iter().filter(|v| *v & (1u32 << digits - i) > 0).count();
        if c >= (oxy.len() as f32 / 2.0).ceil() as usize {
            oxy = oxy.into_iter().filter(|v| v & (1u32 << digits - i) > 0).collect();
        } else {
            oxy = oxy.into_iter().filter(|v| !v & (1u32 << digits - i) > 0).collect();
        }
        if oxy.len() == 1 {
            break;
        }
    }

    for i in 0..=digits {
        let c = co2.iter().filter(|v| (*v & (1u32 << digits - i)) > 0).count();
        if c < (co2.len() as f32 / 2.0).ceil() as usize {
            co2 = co2.into_iter().filter(|v| v & (1u32 << digits - i) > 0).collect();
        } else {
            co2 = co2.into_iter().filter(|v| !v & (1u32 << digits - i) > 0).collect();
        }
        if co2.len() == 1 {
            break;
        }
    }

    dbg!(oxy[0], co2[0], oxy[0] * co2[0]);

    Ok(())
}