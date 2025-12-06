#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Debug, Clone, Copy)]
struct Range {
    from: u64,
    to: u64
}

#[derive(Debug)]
struct RangeCollection {
    coll: Vec<Range>
}

impl Range {
    fn from_string(s: String) -> Range {
        let (left, right) = s.split_once('-').expect("Invalid range string provided");
        
        Range {
            from: left.parse::<u64>().expect("Range 'from' value is not a number"),
            to: right.parse::<u64>().expect("Range 'to' value is not a number")
        }
    }

    fn merge_range_before(&mut self, range_left: &Range) -> Result<(), String> {
        if range_left.to + 1 >= self.from {
            self.from = self.from.min(range_left.from);
            return Ok(());
        }
        Err("Could not merge ranges as there are gaps between them".to_string())
    }

    fn merge_range_after(&mut self, range_right: &Range) -> Result<(), String> {
        if range_right.from - 1 <= self.to {
            self.to = self.to.max(range_right.to);
            return Ok(());
        }
        Err("Could not merge ranges as there are gaps between them".to_string())
    }

    fn in_range(&self, num: u64) -> bool {
        num >= self.from && num <= self.to
    }
}

impl RangeCollection {
    fn new() -> RangeCollection {
        RangeCollection {
            coll: Vec::new()
        }
    }

    fn insert(&mut self, mut range: Range) {
        let mut i = self.coll.iter().position(|r| range.from <= r.from).unwrap_or(self.coll.len());
        if i > 0 && range.merge_range_before(&self.coll[i - 1]).is_ok() {
            self.coll.remove(i - 1);
            i -= 1;
        }
        if i < self.coll.len() && range.merge_range_after(&self.coll[i]).is_ok() {
            self.coll.remove(i);
        }
        self.coll.insert(i, range);
    }

    fn is_in_range(&self, num: u64) -> bool {
        self.coll.iter().any(|range| range.in_range(num))
    }

    fn get_range_count(&self) -> u64 {
        self.coll.iter().fold(0, |acc, r| acc + r.to - r.from + 1)
    }
}

fn main() {
    let mut fresh_count: u64 = 0;

    let file = File::open("input.txt").unwrap();
    let mut ranges = RangeCollection::new();
    let mut ranges_ready = false;

    for line in BufReader::new(file).lines() {
        let ln = line.unwrap();
        if ln == "" {
            ranges_ready = true;
            continue;
        }
        if ranges_ready {
            fresh_count += ranges.is_in_range(ln.parse::<u64>().unwrap()) as u64;
        } else {
            ranges.insert(Range::from_string(ln));
        }
    }
    println!("----");
    for i in &ranges.coll {
        println!("{} - {}", i.from, i.to);
    }
   
    println!("Fresh ingredients: {}", fresh_count);
    println!("Possible fresh IDs: {}", ranges.get_range_count());
}
