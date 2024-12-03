use std::collections::HashMap;
use anyhow::anyhow;

use super::{read_input_file, SolveAdvent};

pub struct Day1;

fn preprocess_lists(path_to_file: &str) -> anyhow::Result<(Vec<i64>, Vec<i64>)> {
    //! Performs the preprocessing to read the `.txt` file, and parse the strings into integers.
    //! Returns the two lists sorted!
    let file_contents = read_input_file(path_to_file)?;
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in file_contents.lines() {
        let [num1, num2]: [&str; 2] = line.trim().split(' ').filter(|substr| !substr.is_empty()).collect::<Vec<_>>().try_into().map_err(|_| anyhow!("Failed to coerce to length 2 array!"))?;
        let num1 = num1.parse::<i64>()?;
        let num2 = num2.parse::<i64>()?;
        list1.push(num1);
        list2.push(num2);
    }
    list1.sort();
    list2.sort();
    Ok((list1, list2))
}


impl SolveAdvent for Day1 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        //! We just need to sum the absolute difference between the two lists.
    let (list1, list2) = preprocess_lists(path_to_file)?;
    let mut running_delta = 0;
    for (n1, n2) in list1.into_iter().zip(list2.into_iter()) {
        running_delta += (n2 - n1).abs()
    }
    println!("Running delta is {:?}", running_delta);
     Ok(())   
    }

    fn solve_part2(path_to_file: &str) -> anyhow::Result<()> {
        //! Multiply each value in `list1` but how often it occurs in `list2`.
        //! A hashmap trivially handles this situation.
        let (list1, list2) = preprocess_lists(path_to_file)?;
        let mut list2_counter: HashMap<i64, i64> = HashMap::new();
        for num2 in list2.iter() {
            *list2_counter.entry(*num2).or_default() += 1;
        }
        let mut similarity_score = 0;
        for num1 in list1 {
            if let Some(count) = list2_counter.get(&num1) {
                similarity_score += *count * num1;
            }
        }
        println!("Similarity score is {:?}", similarity_score);
        Ok(())
    }
}