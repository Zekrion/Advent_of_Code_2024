use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct DiffCounts {
    negatives: usize,
    zeros: usize,
    positives: usize,
}

fn is_valid_pattern(levels: &[i32]) -> bool {

    let diffs: Vec<i32> = levels.windows(2).map(|w| w[0] - w[1]).collect();

    // Check if differences are monotonic (all positive or all negative)
    let is_monotonic = diffs.iter().all(|&d| d > 0) || diffs.iter().all(|&d| d < 0);

    // Check if differences are within the allowed range
    let is_in_range = diffs.iter().all(|&d| (0 < d.abs() && d.abs() <= 3));

    is_monotonic && is_in_range
}

fn count_diffs(diffs: &Vec<i32>) -> DiffCounts {
    let mut diff_counts = DiffCounts {
        negatives: 0,
        zeros: 0,
        positives: 0,
    };

    for &diff in diffs {
        if diff < 0 {
            diff_counts.negatives += 1;
        } else if diff == 0 {
            diff_counts.zeros += 1;
        } else {
            diff_counts.positives += 1;
        }
    }

    return diff_counts
}

fn get_monotonic_majority(diff_counts: DiffCounts) -> String {
    // get monotonic pattern by majority
    let monotonic_majority;
    if diff_counts.negatives >= diff_counts.zeros && diff_counts.negatives >= diff_counts.positives {
         monotonic_majority = "neg";
    } else if diff_counts.positives >= diff_counts.zeros {
         monotonic_majority = "pos";
    } else {
         monotonic_majority = "zeros";
    };

    return monotonic_majority.to_string();
}

fn is_valid_pattern2(levels: &Vec<i32>) -> bool {

    let diffs: Vec<i32> = levels.windows(2).map(|w| w[0] - w[1]).collect();

    let mut faulty_diffs: Vec<usize> = Vec::new();

    let diff_counts = count_diffs(&diffs);

    let monotonic_majority = get_monotonic_majority(diff_counts);

    // look for faulty differences based on found monotonic majority
    for i in 0..diffs.len() {
        if monotonic_majority == "pos" {
            if diffs[i] <= 0 || diffs[i] > 3 {
                faulty_diffs.push(i);
            }
        } else if monotonic_majority == "neg" {
            if diffs[i] >= 0 || diffs[i] < -3 {
                faulty_diffs.push(i);
            }
        } else {
            faulty_diffs.push(i);
        }
    }
    
    // if more than two faulty uncorrectible
    if faulty_diffs.len() > 2 {
        return false;
    }

    // if exactly two they must be together otherwise uncorrectible. I together you have to remove the middle level element
    if faulty_diffs.len() == 2 {
        if faulty_diffs[1] - faulty_diffs[0] > 1 {
            return false;
        }
        let filtered_levels: Vec<i32> = [&levels[..faulty_diffs[1]], &levels[faulty_diffs[1] + 1..]].concat();
        return is_valid_pattern(&filtered_levels)
    }

    // if exactly one diff you need to check both the current and the following level
    if faulty_diffs.len() == 1 {
        let filtered_levels: Vec<i32> = [&levels[..faulty_diffs[0]], &levels[faulty_diffs[0] + 1..]].concat();
        let  valid = is_valid_pattern(&filtered_levels);
        if !valid {
            let filtered_levels: Vec<i32> = [&levels[..faulty_diffs[0] + 1], &levels[faulty_diffs[0] + 2..]].concat();
            return is_valid_pattern(&filtered_levels)
        }
        return true;
    }

    // println!("Monotonic Majority: {}, Levels: {:?}, Diffs: {:?}, Faulty: {:?}",monotonic_majority, levels, diffs, faulty_diffs);

    return false;
}

// ##############################################################################################

fn sub1(data: &[Vec<i32>]) -> usize {
    data.iter().filter(|&levels| is_valid_pattern(levels)).count()
}

fn sub2(data: &[Vec<i32>]) -> usize {
    let mut count = 0;

    for levels in data {

        if is_valid_pattern(levels) || is_valid_pattern2(levels) {
            count += 1;
        }
    }

    return count
}

fn main() -> io::Result<()> {
    // Open and read the file
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let levels: Vec<Vec<i32>> = reader
        .lines()
        .filter_map(|line| line.ok()) 
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok()) 
                .collect()
        })
        .collect();

    // Part 1 solution
    let result_sub1 = sub1(&levels);
    println!("Sub 1: There are {} safe patterns.", result_sub1);

    // Part 2 solution
    let result_sub2 = sub2(&levels);
    println!("Sub 2: There are {} safe or tolerable patterns.", result_sub2);

    Ok(())
}
