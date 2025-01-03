use std::collections::{HashMap, HashSet};

use super::{read_input_file, SolveAdvent};

pub struct Day22;

fn mix_and_prune(secret_number: i64, mix_value: i64) -> i64 {
    (mix_value ^ secret_number) % 16777216
}

fn find_nth_secret_number(first_number: i64, n: usize) -> i64 {
    //! Find the `n`th secret number as directed in the problem.
    let mut secret_number = first_number;
    for _ in 0..n {
        secret_number = mix_and_prune(secret_number, secret_number * 64);
        secret_number = mix_and_prune(secret_number, secret_number / 32);
        secret_number = mix_and_prune(secret_number, secret_number * 2048);
    }
    secret_number
}

type PriceChangeMap = HashMap<[i64; 4], i64>;

fn generate_price_changes_map(starting_number: i64, max_price_changes: usize) -> anyhow::Result<PriceChangeMap> {
    //! Generate a map of groupings of 4 price deltas mapped to the sales price (the price the banans would sell for 
    //! if sold after the given 4 price delta sequence is seen by the monkey.)
    let mut secret_numbers = vec![starting_number];
    let mut secret_number = starting_number;
    for _ in 0..max_price_changes {
        secret_number = find_nth_secret_number(secret_number, 1);
        secret_numbers.push(secret_number);      
    }
    let digits = secret_numbers.into_iter().map(|num| num.to_string().chars().last().unwrap().to_digit(10).unwrap() as i64).collect::<Vec<_>>();
    let deltas = digits.iter().zip(digits.iter().skip(1)).map(|(num1, num2)| *num2 - *num1).collect::<Vec<_>>();
    let mut price_change_map = HashMap::new();
    let mut lower = 0;
    //Slide the window of size 4 1 at a time, and record the window deltas mapped to the corresponding digits value.
    while let Some(price_delta) = deltas.get(lower..lower + 4) {
        let deltas: [i64; 4] = price_delta.try_into().map_err(|_| anyhow::anyhow!("Failed to coerce to length 4 array"))?;
        if let Some(sales_price) = digits.get(lower + 4){
            //or_insert is used because only the first iteration of a sequence of 4 deltas
            //will ever be seen by the monkey, so it is imperative to prevent repeat sequences
            //from overwriting any values already in the map.
            price_change_map.entry(deltas).or_insert(*sales_price);
        }
        lower += 1;
    }
    Ok(price_change_map)
}

impl SolveAdvent for Day22 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        //! Simply apply the formula the correct number of times.
        let secret_number_iterations = 2000;
        let file_contents = read_input_file(path_to_file)?;
        let starting_numbers = file_contents.trim().lines().map(|num| num.parse::<i64>()).collect::<Result<Vec<_>, _>>()?;
        let mut total_secret_numbers = 0;
        for starting_number in starting_numbers {
            total_secret_numbers += find_nth_secret_number(starting_number, secret_number_iterations);
        }
        println!("Total of all secret numbers after {} iterations is: {}", secret_number_iterations, total_secret_numbers);
        Ok(())
    }
    fn solve_part2(path_to_file: &str) -> anyhow::Result<()> {
        //! Index each of the 2001 secret numbers price deltas to the corresponding sales price.
        //! Then iterate over the unique 4-delta groupings, and compute the max possible number of bananas sold.
        let file_contents = read_input_file(path_to_file)?;
        let secret_number_iterations = 2000;
        let starting_numbers = file_contents.trim().lines().map(|num| num.parse::<i64>()).collect::<Result<Vec<_>, _>>()?;
        let price_change_maps = starting_numbers.into_iter().map(|starting_number| generate_price_changes_map(starting_number, secret_number_iterations)).collect::<Result<Vec<_>, _>>()?;
        let all_price_changes = price_change_maps.iter().flat_map(|price_change_map| price_change_map.keys()).collect::<HashSet<_>>();
        let mut max_possible_bananas = 0;
        for price_change_sequence in all_price_changes {
            let total_bananas = price_change_maps.iter().map(|price_change_map| price_change_map.get(price_change_sequence).unwrap_or(&0)).sum::<i64>();
            max_possible_bananas = max_possible_bananas.max(total_bananas);
        }
        println!("Max possible bananas for a single 4 price change is {}", max_possible_bananas);
        Ok(())
    }
}