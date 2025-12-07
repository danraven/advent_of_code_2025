use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};


const CHAR_START: char = 'S';
const CHAR_SPLIT: char = '^';
const FILE_INPUT: &str = "input.txt";

fn main() {
    let mut splits: u32 = 0;
    let mut tachyons: HashSet<usize> = HashSet::new();
    let file = File::open(FILE_INPUT).unwrap();

    for line in BufReader::new(file).lines() {
        for (idx, chr) in line.unwrap().chars().enumerate() {
            if chr == CHAR_START {
                tachyons.insert(idx);
                continue;
            }
            if chr == CHAR_SPLIT && tachyons.contains(&idx) {
                tachyons.remove(&idx);
                tachyons.insert(idx - 1);
                tachyons.insert(idx + 1);
                splits += 1;
            }
        }
    }

    println!("There are {} splits", splits);
}
