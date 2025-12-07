use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, BufRead};


const CHAR_START: char = 'S';
const CHAR_SPLIT: char = '^';
const FILE_INPUT: &str = "input.txt";

fn main() {
    let mut splits: u32 = 0;
    let mut tachyons: BTreeMap<usize, u64> = BTreeMap::new();
    let file = File::open(FILE_INPUT).unwrap();

    for line in BufReader::new(file).lines() {
        for (idx, chr) in line.unwrap().chars().enumerate() {
            if chr == CHAR_START {
                tachyons.insert(idx, 1);
                continue;
            }
            if chr == CHAR_SPLIT && tachyons.contains_key(&idx) {
                let parent = tachyons[&idx].clone();
                *tachyons.entry(idx - 1).or_insert(0) += parent;
                *tachyons.entry(idx + 1).or_insert(0) += parent;
                tachyons.remove(&idx);
                splits += 1;
            }
        }
    }
    let timelines: u64 = tachyons.values().sum();

    println!("There are {} splits", splits);
    println!("There are {} timelines", timelines);
}
