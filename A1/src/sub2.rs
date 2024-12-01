use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn run() {
    let file = File::open("input.txt").expect("Error opening file");
    let reader = BufReader::new(file);

    let mut vec_0: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();

    // Read the input file and append to the vector and map
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let contents: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap_or(0)).collect();
                vec_0.push(contents[0]);

                // insert or increment the count in the map
                *map.entry(contents[1]).or_insert(0) += 1;
            }
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
    }

    let mut score = 0 ;

    // expecting the vectors to be of the same length
    for element in vec_0.iter() {
        score += element * *map.entry(*element).or_insert(0)
    }

    println!("The similarity score for the two lists is: {}", score);
}