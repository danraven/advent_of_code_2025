#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};


enum Direction {
    Left(u16),
    Right(u16)
}

fn parse_line(line: String) -> Result<Direction, String> {
    let units = match line[1..].parse::<u16>() {
        Ok(val) => val,
        Err(e) => return Err(e.to_string())
    };
    match line.chars().nth(0) {
        Some('L') => Ok(Direction::Left(units)),
        Some('R') => Ok(Direction::Right(units)),
        None => Err(String::from("No direction provided")),
        _ => Err(format!("Unknown direction '{}', acceptable directions are 'L' and 'R'", line.chars().nth(0).unwrap()))
    }
}

fn rotate_dial_v1(dial: &mut u8, direction: Direction) -> u8 {
    match direction {
        Direction::Left(dist) => {
            // Cut the rotation units with mod, every 100 units are a full rotation and can be
            // disregarded.
            // Start from 100, since we're subtracting units and instead of dipping into negatives,
            // we need to count down from 100.
            // Lastly we use mod again in case we don't go below 0 (100).
            *dial = ((100 + (*dial as i16 - dist as i16 % 100)) % 100) as u8;
        },
        Direction::Right(dist) => {
            *dial = ((*dial as u16 + dist) % 100) as u8;
        }
    };
    (*dial == 0) as u8
}

fn rotate_dial_v2(dial: &mut u8, direction: Direction) -> u8 {
    let before = *dial;

    match direction {
        Direction::Left(dist) => {
            // Cut the rotation units with mod, every 100 units are a full rotation and can be
            // disregarded.
            // Start from 100, since we're subtracting units and instead of dipping into negatives,
            // we need to count down from 100.
            // Lastly we use mod again in case we don't go below 0 (100).
            *dial = ((100 + (*dial as i16 - dist as i16 % 100)) % 100) as u8;

            // Every 100 click is a full rotation. If the new value is higher OR zero, that's also
            // a hit, unless the starting value was also zero, in which case the full rotation
            // count takes care of it.
            (dist / 100 + ((*dial > before || *dial == 0) && before != 0) as u16) as u8
        },
        Direction::Right(dist) => {
            *dial = ((*dial as u16 + dist) % 100) as u8;

            // Every 100 click is a full rotation. If the new value is lower (this includes landing
            // on zero), that's also a hit.
            (dist / 100 + (*dial < before) as u16) as u8
        }
    }
}

fn main() {
    let mut dial: u8 = 50;
    let mut pwd: u32 = 0;
    let file = File::open("input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        // pwd += rotate_dial_v1(&mut dial, parse_line(line.unwrap()).unwrap()) as u32;
        pwd += rotate_dial_v2(&mut dial, parse_line(line.unwrap()).unwrap()) as u32;
    }

    println!("The password is: {}", pwd);
}
