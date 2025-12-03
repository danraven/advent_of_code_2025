#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};


fn highest_joltage(line: String) -> u8 {
    let mut joltage: (u8, u8) = (0, 0);
    let batteries = Vec::from_iter(line.chars());
    for c in &batteries[..batteries.len() - 1] {
        let val = c.to_digit(10).unwrap() as u8;
        if val > joltage.0 {
            joltage.0 = val;
            joltage.1 = 0;
        } else if val > joltage.1 {
            joltage.1 = val;
        }
    }
    let last = batteries.last().unwrap().to_digit(10).unwrap() as u8;
    if last > joltage.1 {
        joltage.1 = last;
    }

    joltage.0 * 10 + joltage.1
}

fn main() {
    let mut output: u32 = 0;

    let file = File::open("input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        let batteries = line.unwrap();
        output += highest_joltage(batteries) as u32;
    }
    println!("Total output joltage: {}", output);
}
