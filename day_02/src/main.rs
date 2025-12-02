use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn check_invalid_ids_v1(id_start: &str, id_end: &str) -> u64 {
    let mut invalids: HashSet<u64> = HashSet::new();
    let num_start = id_start.parse::<u64>().expect("Couldn't parse start ID");
    let num_end = id_end.parse::<u64>().expect("Couldn't parse end ID");
    for i in num_start..num_end {
        let i_string = i.to_string();
        let chunks = i_string.split_at_checked(i_string.len() / 2);
        match chunks {
            Some((half1, half2)) => {
                if half1 == half2 {
                    invalids.insert(i);
                }
            },
            None => {
                continue;
            }
        };
    }

    invalids.into_iter().sum()
}

fn main() {
    let mut reader = BufReader::new(File::open("input.txt").unwrap());
    let mut buf: Vec<u8> = Vec::new();
    let mut invalids: u64 = 0;

    while reader.read_until(b',', &mut buf).expect("File read failed") != 0 {
        let mut chars = buf.clone();
        chars.pop();
        let pair = String::from_utf8(chars).unwrap();
        let (id_start, id_end) = pair.split_once('-').expect("Invalid ID pair");
        invalids += check_invalid_ids_v1(id_start, id_end);

        buf.clear();
    }

    println!("Sum of invalid IDs: {}", invalids);
}
