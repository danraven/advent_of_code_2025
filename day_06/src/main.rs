use std::fs::File;
use std::io::{BufRead, BufReader, Seek};
use regex::Regex;


#[derive(Debug)]
enum Operation {
    Add,
    Mul
}

#[derive(Debug)]
struct Calculation {
    nums: Vec<String>,
    operation: Option<Operation>
}

impl Calculation {
    fn new(num_count: usize) -> Calculation {
        Calculation {
            nums: vec![String::new(); num_count],
            operation: None
        }
    }

    fn add_digit(&mut self, idx: usize, digit: &char) -> Result<(), String> {
        if digit.is_whitespace() {
            return Ok(());
        }
        if digit.to_digit(10).is_none() {
            return Err("Attempt at adding non-digit character to number".to_string());
        }
        self.nums.get_mut(idx).ok_or("Number index not found")?.push(*digit);
        Ok(())
    }
    
    fn set_operation(&mut self, op: Operation) {
        self.operation = Some(op);
    }

    fn set_operation_char(&mut self, chr: &char) -> Result<(), String> {
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
            Operation::Add => self.nums.iter().fold(0, |acc, n| acc + n.parse::<i64>().unwrap()),
            Operation::Mul => self.nums.iter().fold(1, |acc, n| acc * n.parse::<i64>().unwrap())
        }
    }
}

fn main() {
    let mut calcs: Vec<Calculation> = Vec::new();

    let mut file = File::open("input.txt").unwrap();
    let re = Regex::new(r"[\*\+]\s+").unwrap();

    let last_line = BufReader::new(&file).lines().last().unwrap().unwrap();
    for op in re.find_iter(&last_line) {
        let chars = op.as_str().chars();
        let mut calc = Calculation::new(chars.count() - (op.end() < last_line.len()) as usize);
        let _ = calc.set_operation_char(&op.as_str().chars().nth(0).unwrap());
        calcs.push(calc);
    }

    let _ = file.rewind();

    for line in BufReader::new(&file).lines()
        .take_while(|l| {
            let chr = l.as_ref().unwrap().chars().nth(0).unwrap();
            chr != '*' && chr != '+'
        }) {
        let nums_string = line.unwrap();
        let mut idx: usize = 0;
        for calc in &mut calcs {
            for (j, chr) in nums_string[idx..idx + calc.nums.len()].chars().enumerate() {
                let _ = calc.add_digit(j, &chr);
            }
            idx += calc.nums.len() + 1;
        }
    }

    let total = calcs.iter().fold(0, |acc, c| acc + c.calculate());
   
    println!("Total: {}", total);
}
