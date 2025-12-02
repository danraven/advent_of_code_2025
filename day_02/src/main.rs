#![allow(dead_code)]

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn check_invalid_ids(id_start: &str, id_end: &str) -> u64 {
    let mut invalids: HashSet<u64> = HashSet::new();
    let num_start = id_start.parse::<u64>().expect("Couldn't parse start ID");
    let num_end = id_end.parse::<u64>().expect("Couldn't parse end ID");
    for i in num_start..num_end {
        if check_id_v2(&i) {
            invalids.insert(i);
        }
    }

    invalids.into_iter().sum()
}

fn check_id_v1(id: &u64) -> bool {
    let id_string = id.to_string();
    if let Some((half1, half2)) = id_string.split_at_checked(id_string.len() / 2) {
        return half1 == half2;
    }
    false
}

fn check_id_v2(id: &u64) -> bool {
    let id_string = id.to_string();
    for i in 1..(id_string.len() / 2 + 1) {
        let pattern = &id_string[0..i];
        let size = pattern.len();
        let mut ptr = i;
        while ptr + size <= id_string.len() && &id_string[ptr..(ptr + size)] == pattern {
            ptr += size;
        }
        if ptr == id_string.len() {
            return true
        }
    }
    false
}

fn main() {
    let mut reader = BufReader::new(File::open("input.txt").unwrap());
    let mut buf: Vec<u8> = Vec::new();
    let mut invalids: u64 = 0;

    while reader.read_until(b',', &mut buf).expect("File read failed") != 0 {
        let mut chars = buf.clone();
        chars.pop(); // remove comma from string
        let pair = String::from_utf8(chars).unwrap();
        let (id_start, id_end) = pair.split_once('-').expect("Invalid ID pair");
        invalids += check_invalid_ids(id_start, id_end);

        buf.clear();
    }

    println!("Sum of invalid IDs: {}", invalids);
}
