use std::fs;

fn parse_input(input: &str, part2: bool) -> i32 {
    const MUL_KEYWORD: &str = "mul(";
    let mut mul_keyword_flag: bool = false;
    let mut mul_keyword_counter = 0;

    const DO_KEYWORD: &str = "do()";
    let mut do_keyword_flag: bool = true;
    let mut do_keyword_counter = 0;

    const DONT_KEYWORD: &str = "don't()";
    let mut dont_keyword_counter = 0;
    
    let mut number_counter = 0;
    let mut first_number = String::new();
    let mut second_number = String::new();
    let mut first_number_complete = false;
    let mut second_number_complete = false;

    let mut result = 0;

    for ch in input.chars() {
        // only check for do() and don't() if part 2
        if part2 {
            if !do_keyword_flag && ch == DO_KEYWORD.chars().nth(do_keyword_counter).unwrap() {
                do_keyword_counter += 1;
            } else {
                do_keyword_counter = 0;
            }
            if do_keyword_counter == DO_KEYWORD.len() {
                do_keyword_flag = true;
                do_keyword_counter = 0;
                continue;
            }

            if do_keyword_flag && ch == DONT_KEYWORD.chars().nth(dont_keyword_counter).unwrap() {
                dont_keyword_counter += 1;
            } else {
                dont_keyword_counter = 0;
            }
            if dont_keyword_counter == DONT_KEYWORD.len() {
                do_keyword_flag = false;
                dont_keyword_counter = 0;
                continue;
            }
        }

        // without do flag stop here
        if !do_keyword_flag {
            continue;
        }

        //
        if !mul_keyword_flag && ch == MUL_KEYWORD.chars().nth(mul_keyword_counter).unwrap() {
            mul_keyword_counter += 1;
        } else {
            mul_keyword_counter = 0;
        }
        if mul_keyword_counter == MUL_KEYWORD.len() {
            mul_keyword_flag = true;
            mul_keyword_counter = 0;
            continue;
        }

        // without "mul(" keyword flag stop here
        if mul_keyword_flag {
            // completion of first number
            if !first_number_complete && number_counter > 0 && ch == ','{
                first_number_complete = true;
                number_counter = 0;
            }
            // completion of second number
            else if first_number_complete && number_counter > 0 && ch == ')' {
                second_number_complete = true;
                number_counter = 0;
            }
            // collect numbers
            else if ('0'..='9').contains(&ch) && number_counter < 3 {
                if number_counter == 0  && ch == '0' {
                    // reset state
                    mul_keyword_flag = false;
                    first_number_complete = false;
                    second_number_complete = false;
                    first_number.clear();
                    second_number.clear();
                }
                else if !first_number_complete {
                    first_number.push(ch);
                    number_counter += 1;
                } else {
                    second_number.push(ch);
                    number_counter += 1;
                }
            }
            // if invalid character encountered, reset state
            else {
                // reset state
                number_counter = 0;
                mul_keyword_flag = false;
                first_number_complete = false;
                second_number_complete = false;
                first_number.clear();
                second_number.clear();
            }
        }
        
        // if both numbers are collected, perform multiplication and reset state
        if second_number_complete {
            let num1: i32 = first_number.parse().expect("Invalid first number");
            let num2: i32 = second_number.parse().expect("Invalid second number");
            result += num1 * num2;

            // reset state
            number_counter = 0;
            mul_keyword_flag = false;
            first_number_complete = false;
            second_number_complete = false;
            first_number.clear();
            second_number.clear();
        }
    }
    return result;
}

fn main() -> std::io::Result<()> {
    // Path to your text file
    let file_path = "input.txt";
    
    // Read the entire file into a single String
    let content = fs::read_to_string(file_path)?;

    let result1 = parse_input(&content, false);
    
    println!("Part1: {}", result1);

    let result2 = parse_input(&content, true);

    println!("Part2: {}", result2);
    
    Ok(())
}