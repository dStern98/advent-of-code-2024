use anyhow::Context;

use super::{read_input_file, SolveAdvent};

pub struct Day11;

fn blink(num_in: i64) -> Vec<i64> {
    //! Blink the stone by way of straightforward application 
    //! of the rules.
    if num_in == 0 {
        //Stone with engraved number 0 becomes 1
        return vec![1];
    }
    let num_in_as_str = num_in.to_string();
    if num_in_as_str.len() % 2 == 0 {
        //If stone number has an even number of digits, split into two stones
        let num1 = &num_in_as_str[0..num_in_as_str.len() / 2];
        let mut num2 = &num_in_as_str[num_in_as_str.len() / 2 ..];
        if num2.starts_with('0') {
            //leading zeros are dropped
            if let Some(start_of_non_zero_idx) =  num2.find(|char| char > '0' && char <= '9') {
                num2 = &num2[start_of_non_zero_idx..];
            } else {
                num2 = "0";
            }
        }
        return vec![num1.parse::<i64>().unwrap(), num2.parse::<i64>().unwrap()];

    }
    //Replace with one number multiplied by 2024
    vec![2024 * num_in]
}

impl SolveAdvent for Day11 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        //! 25 blinks turns out to be low enough that the exponential average growth
        //! of the `stones` array has not yet become a problem.
        let file_contents = read_input_file(path_to_file)?;
        let stones: Result<Vec<_>, _> = file_contents.trim().split(' ').map(|num| num.trim().parse::<i64>()).collect();
        let mut stones = stones.context("Failed to process input into numbers")?;
        for _ in 0..25 {
            let old_stones = std::mem::take(&mut stones);
            for old_stone in old_stones {
                stones.extend(blink(old_stone));
            }
        }
        println!("After 25 blinks, there are {} stones", stones.len());
        Ok(())
    }

    fn solve_part2(_path_to_file: &str) -> anyhow::Result<()> {
        //! Brute force solution on 75 blinks fails to complete.
        Ok(())
    }
}