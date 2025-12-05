#![allow(dead_code)]

use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;


const MAX_ADJACENCY: u8 = 4;

fn remove_accessible_rolls(lines: &mut VecDeque<Vec<i8>>, num_adjacent: u8) -> u64 {
    let mut total_removed: u64 = 0;
    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            if lines[row][col] > -1 && lines[row][col] < num_adjacent as i8 {
                lines[row][col] = -1;
                total_removed += 1;
            }
        }
    }

    total_removed
}

fn check_adjacency(lines: &mut VecDeque<Vec<i8>>) {
    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            if lines[row][col] < 0 {
                continue;
            }
            lines[row][col] = 0;
            for r in row.checked_sub(1).unwrap_or(0)..row.add(2).min(lines.len()) {
                for c in col.checked_sub(1).unwrap_or(0)..col.add(2).min(lines[row].len()) {
                    if lines[r][c] >= 0 && !(r == row && c == col) {
                        lines[row][col] += 1;
                    }
                }
            }
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
    }

    loop {
        check_adjacency(&mut lines);
        let removed = remove_accessible_rolls(&mut lines, MAX_ADJACENCY);
        rolls += removed;
        if removed == 0 {
            break;
        }
    }

    println!("Accessible paper rolls: {}", rolls);
}
