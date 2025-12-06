use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Debug)]
enum Operation {
    Add,
    Mul
}

#[derive(Debug)]
struct Calculation {
    nums: Vec<i64>,
    operation: Option<Operation>
}

impl Calculation {
    fn new() -> Calculation {
        Calculation {
            nums: Vec::new(),
            operation: None
        }
    }

    fn add(&mut self, num: i64) {
        self.nums.push(num);
    }
    
    fn set_operation(&mut self, op: Operation) {
        self.operation = Some(op);
    }

    fn set_operation_char(&mut self, chr: char) -> Result<(), String> {
        match chr {
            '+' => self.set_operation(Operation::Add),
            '*' => self.set_operation(Operation::Mul),
            _ => return Err("Invalid operation character".to_string())
        };

        Ok(())
    }

    fn calculate(&self) -> i64 {
        let op = self.operation.as_ref().expect("No operation is set");
        match op {
            Operation::Add => self.nums.iter().copied().reduce(|acc, n| acc + n).unwrap_or(0),
            Operation::Mul => self.nums.iter().copied().reduce(|acc, n| acc * n).unwrap_or(0)
        }
    }
}

fn main() {
    let mut calcs: Vec<Calculation> = Vec::new();

    let file = File::open("input.txt").unwrap();

    for line in BufReader::new(file).lines() {
        for (i, part) in line.unwrap().split_whitespace().enumerate() {
            if i == calcs.len() {
                calcs.push(Calculation::new());
            }
            let c = calcs.get_mut(i).unwrap();
            if let Ok(num) = part.parse::<i64>() {
                c.add(num);
            } else {
                let _ = c.set_operation_char(part.chars().nth(0).unwrap());
            }
        }
    }

    let total = calcs.iter().fold(0, |acc, c| acc + c.calculate());
   
    println!("Total: {}", total);
}
