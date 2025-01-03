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

impl SolveAdvent for Day22 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
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
    fn solve_part2(_path_to_file: &str) -> anyhow::Result<()> {
        Ok(())
    }
}