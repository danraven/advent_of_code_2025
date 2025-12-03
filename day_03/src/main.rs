#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};


fn highest_joltage(line: String, cells: usize) -> u64 {
    let mut joltage: Vec<u8> = vec![0;cells];
    let batteries = Vec::from_iter(line.chars());
    let length = batteries.len();
    if cells > length {
        panic!("More cells requested than available");
    }

    for i in 0..length {
        let available = (length as i8 - cells as i8 - i as i8).min(0).abs() as usize;
        let val = batteries[i].to_digit(10).unwrap() as u8;

        for j in available..cells {
            if val > joltage[j] {
                joltage[j] = val;
                if joltage.get(j + 1).is_some() {
                    joltage[j + 1] = 0;
                }
                break;
            }
        }
    }
    joltage.iter().enumerate()
        .map(|(idx, digit)| {
            10_u64.pow((cells - idx - 1) as u32) * *digit as u64
        })
        .sum()
}

fn main() {
    let mut output: u64 = 0;

    let file = File::open("input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        let batteries = line.unwrap();
        output += highest_joltage(batteries, 12);
    }
    println!("Total output joltage: {}", output);
}
