use std::{fs::File};

use std::io::{self, prelude::*, BufReader};

use std::{iter};
use std::collections::HashSet;
use std::collections::VecDeque;
use regex::{Regex};


fn main() -> io::Result<()> {
    let file = File::open("data/q")?;
    let reader = BufReader::new(file);
    Ok(())
}