use std::{fs::File};

use std::io::{self, prelude::*, BufReader};

use std::{iter};
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashSet;
use std::collections::VecDeque;
use std::mem::replace;
use std::rc::Rc;
use regex::{Regex};


struct Sample {
    a: i32,
    b: i32,
}

struct Train<'a> {
    c: &'a mut Sample,
}


fn main() -> io::Result<()> {
    let a = [3, 4, 5];
    a.iter


    Ok(())
}