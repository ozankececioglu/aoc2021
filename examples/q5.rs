use std::{fs::File};

use std::io::{self, prelude::*, BufReader};
use std::convert::TryInto;
use arrayvec::ArrayVec;

use std::{iter};
use std::collections::{HashMap, HashSet, VecDeque};
use regex::{Regex};


fn main() -> io::Result<()> {
    let file = File::open("data/q5")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    Ok(())
}