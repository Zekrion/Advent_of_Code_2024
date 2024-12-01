use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    let file = File::open("input.txt").expect("Error opening file");
    let reader = BufReader::new(file);

    let mut vec_0: Vec<i32> = Vec::new();
    let mut vec_1: Vec<i32> = Vec::new();

    // Read the input file and append to the vectors
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let contents: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap_or(0)).collect();
                vec_0.push(contents[0]);
                vec_1.push(contents[1]);
            }
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
    }

    // Sort both vectors
    vec_0.sort();
    vec_1.sort();

    let mut diff = 0 ;

    // Expecting the vectors to be of the same length
    for i in 0..vec_0.len() {
        diff += (vec_0[i] - vec_1[i]).abs();
    }

    println!("The difference in character counts is: {}", diff);
}