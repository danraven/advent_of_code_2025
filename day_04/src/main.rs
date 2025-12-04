#![allow(dead_code)]

use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};


const CAPACITY: usize = 2;
const MAX_ADJACENCY: u8 = 4;

fn count_accessible_rolls(rolls: Vec<i8>, num_adjacent: u8) -> u64 {
    rolls.iter().fold(0, |acc, e| acc + (*e > -1 && *e < num_adjacent as i8) as u64)
}

fn check_adjacency(lines: &mut VecDeque<Vec<i8>>) {
    let row = lines.len() - 1;

    for col in 0..lines[row].len() {
        if lines[row][col] < 0 {
            continue;
        }
        if lines[row].get(col.wrapping_sub(1)).is_some_and(|e| *e >= 0) {
            lines[row][col - 1] += 1;
            lines[row][col] += 1;
        }

        if lines.get(row.wrapping_sub(1)).is_none() {
            continue;
        }
        if lines[row - 1].get(col.wrapping_sub(1)).is_some_and(|e| *e >= 0) {
            lines[row - 1][col - 1] += 1;
            lines[row][col] += 1;
        }
        if lines[row - 1].get(col).is_some_and(|e| *e >= 0) {
            lines[row - 1][col] += 1;
            lines[row][col] += 1;
        }
        if lines[row - 1].get(col + 1).is_some_and(|e| *e >= 0) {
            lines[row - 1][col + 1] += 1;
            lines[row][col] += 1;
        }
    }
}

fn main() {
    let mut rolls: u64 = 0;

    let file = File::open("input.txt").unwrap();
    let mut lines: VecDeque<Vec<i8>> = VecDeque::new();

    for line in BufReader::new(file).lines() {
        lines.push_back(line.unwrap().chars()
            .map(|chr| if chr == '@' { 0 } else { -1 })
            .collect()
        );
        if lines.len() > CAPACITY {
            rolls += count_accessible_rolls(lines.pop_front().unwrap(), MAX_ADJACENCY);
        }
        check_adjacency(&mut lines);
    }

    for line in lines {
        rolls += count_accessible_rolls(line, MAX_ADJACENCY);
    }

    println!("Accessible paper rolls: {}", rolls);
}
