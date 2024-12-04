
use anyhow::Context;

use super::{read_input_file, SolveAdvent};

pub struct Day3;

const MUL_WINDOW_SIZE: usize = "Mul(".len();
const DONT_WINDOW_SIZE: usize = "don't()".len();
const DO_WINDOW_SIZE: usize = "do()".len();


fn parse_mul_operation(input: &str) -> anyhow::Result<i64> {
    //! Parse the string that directly follows a `mul(` instruction.
    //! Will return an error as early as one is encountered.
    //! A valid input following the `mul(` string would be this: `5,5)`.
    //! An invalid one looks like this: `6,9!`
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut finished_num1 = false;
    for char in input.chars() {
        if char.is_numeric() {
            if !finished_num1 {
                num1.push(char);
            } else {
                num2.push(char);
            }
        }
        else if char == ',' {
            finished_num1 = true;
        } else if char == ')' {
            //Closed parenthesis marks the end of the mul op
            break;
        } else {
            anyhow::bail!("Encountered illegal character {} while iterating", char);
        }
    }
    let num1 = num1.parse::<i64>().context("Could not turn num1 into an integer")?;
    let num2 = num2.parse::<i64>().context("Could not turn num2 into an integer")?;
    Ok(num1 * num2)
}

impl SolveAdvent for Day3 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        //! Evaluate all valid multiply operations
        let file_contents = read_input_file(path_to_file)?;
        let mut lower_boundary = 0;
        let mut multiplied_sum = 0;
        while let Some(window) = file_contents.get(lower_boundary..lower_boundary + MUL_WINDOW_SIZE) {
            if window == "mul(" {
                let remaining_str = file_contents.get(lower_boundary + MUL_WINDOW_SIZE..).ok_or(anyhow::anyhow!("File slice went out of bounds"))?;
                if let Ok(multiplied_result) = parse_mul_operation(remaining_str) {
                    multiplied_sum += multiplied_result;
                }
            }
            lower_boundary += 1
        }
        println!("Sum of all mul operations is {}", multiplied_sum);
        Ok(())
    }

    fn solve_part2(path_to_file: &str) -> anyhow::Result<()> {
        //! Evaluate all multiply operations as above, but ignore all instructions
        //! after encountering a `don't()` instruction until the next `do()` instruction.
        let file_contents = read_input_file(path_to_file)?;
        let mut multiplied_sum = 0;
        let mut mul_op_enabled = true;
        for lower_boundary in 0..file_contents.len() {
            //While its tempting to use the same window size for all ops, this causes issues at the end of the string parsing
            //when there could be a valid mul op but a dont() op would not fit.
            if let Some(command) = file_contents.get(lower_boundary..lower_boundary + DONT_WINDOW_SIZE) {
                if command == "don't()" {
                    mul_op_enabled = false;
                    continue;
                }
            }
            if let Some(command) = file_contents.get(lower_boundary..lower_boundary + DO_WINDOW_SIZE) {
                if command == "do()" {
                    mul_op_enabled = true;
                    continue;
                }
            }
            if let Some(command) = file_contents.get(lower_boundary..lower_boundary + MUL_WINDOW_SIZE) {
                if command == "mul(" && mul_op_enabled {
                        if let Ok(mul_result) = parse_mul_operation(&file_contents[lower_boundary + MUL_WINDOW_SIZE..]) {
                            multiplied_sum += mul_result;
                        }
                    
                }
            }
        }
        println!("Sum of all enabled multiplications is {}", multiplied_sum);

        Ok(())
    }
}