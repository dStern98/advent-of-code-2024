use std::collections::HashMap;

use anyhow::anyhow;

use super::{read_input_file, SolveAdvent};

pub struct Day19;

///A towel that may or may not be constructed
/// from the `available_stripes`. 
#[derive(Debug, Clone)]
struct Towel<'a> {
    ///The pattern to try and make, such as `brwrr`
    pattern: &'a str,
    ///The available towel patterns to use to make the `pattern`
    available_stripes: &'a Vec<&'a str>,
}


impl <'a> Towel<'a> {
    fn new(pattern: &'a str, available_stripes: &'a Vec<&str>) -> Towel<'a> {
        Towel {
            pattern,
            available_stripes,
        }
    }

    fn count_possibilities(&self, memo: &mut HashMap<&'a str, usize>) -> usize {
        //! Exactly the same algorithm as the `is_possible` method below, except that instead
        //! of returning booleans we return integers.
        if let Some(outcome) =  memo.get(self.pattern) {
            return *outcome;
        }
        if self.pattern.is_empty() {
            //Empty string means we have successfully constructed the pattern
            return 1;
        }
        //Sum all possible ways to make the pattern
        let mut total_possiblities = 0;
        for possible_pattern in self.available_stripes {
            //For every strip pattern that fits, we recursively call with a shorter pattern
            if self.pattern.starts_with(possible_pattern) {
                let new_towel = Towel::new(&self.pattern[possible_pattern.len()..], self.available_stripes);
                total_possiblities +=  new_towel.count_possibilities( memo);
            }
        }
        memo.insert(self.pattern, total_possiblities);
        total_possiblities
    }

    fn is_possible(&self, memo: &mut HashMap<&'a str, bool>) -> bool {
        //! Recursively explore all possible pattern combinations to make the desired `pattern`.
        //! The `memo` is used to memoize results to keep the dynamic programming solution efficient.
        if let Some(outcome) =  memo.get(self.pattern) {
            return *outcome;
        }
        if self.pattern.is_empty() {
            //Empty string means we have successfully constructed the pattern
            return true;
        }
        for possible_pattern in self.available_stripes {
            //For every strip pattern that fits, we recursively call with a shorter pattern
            if self.pattern.starts_with(possible_pattern) {
                let new_towel = Towel::new(&self.pattern[possible_pattern.len()..], self.available_stripes);
                if new_towel.is_possible( memo) {
                    memo.insert(self.pattern, true);
                    return true
                }
            }
        }
        memo.insert(self.pattern, false);
        false
    }
  
}



fn parse_input(file_contents: &str) -> anyhow::Result<(Vec<&str>, Vec<&str>)> {
    //! Process the input file correctly into two vecs, one of the `available_stripes` and one of the `patterns`
    let (available_stripes, patterns) = file_contents.split_once("\r\n").ok_or(anyhow!("Input file is malformed"))?;
    let available_stripes = available_stripes.split(',').map(|pattern| pattern.trim()).collect::<Vec<_>>();
    let patterns = patterns.split_ascii_whitespace().collect::<Vec<_>>();
    Ok((available_stripes, patterns))
}

impl SolveAdvent for Day19 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        //! Solution uses dynamic programming with memoization
        let file_contents = read_input_file(path_to_file)?;
        let (available_stripes, patterns) = parse_input(&file_contents)?;
        let test_patterns = patterns.into_iter().map(|pattern| Towel::new(pattern, &available_stripes)).collect::<Vec<_>>();
        let mut possible_patterns = 0;
        for test_pattern in test_patterns.iter() {
            let mut memo = HashMap::new();
            if test_pattern.is_possible(&mut memo) {
                possible_patterns += 1;
            }
        }
        println!("There are {} possible designs in total", possible_patterns);
        Ok(())
    }

    fn solve_part2(path_to_file: &str) -> anyhow::Result<()> {
        //! Solution uses dynamic programming with memoization
        let file_contents = read_input_file(path_to_file)?;
        let (available_stripes, patterns) = parse_input(&file_contents)?;
        let test_patterns = patterns.into_iter().map(|pattern| Towel::new(pattern, &available_stripes)).collect::<Vec<_>>();
        let mut total_possible_patterns = 0;
        for test_pattern in test_patterns {
            let mut memo = HashMap::new();
            let possibilities = test_pattern.count_possibilities(&mut memo);
            total_possible_patterns += possibilities;
        }
        println!("There are {} different stripe combinations that make the desired pattern", total_possible_patterns);

        Ok(())
    }
}