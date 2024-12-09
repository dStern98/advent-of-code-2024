use std::fs;
use std::path::Path;
use anyhow::Context;


pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

pub fn read_input_file<P>(fp: P) -> anyhow::Result<String>
where
    P: AsRef<Path>,
{
    //! Given a file path, returns the entire file contents as a String.
    let file_contents = fs::read_to_string(fp).context("Could not read file to UTF-8 String.")?;
    Ok(file_contents)
}

///trait representing how to solve the days challenge for the advent calendar.
///Obviously, part1 is for part1 and part2 is for part2.
/// The trait methods do not return anything useful, the answer to the problem
/// should simply be printed.
pub trait SolveAdvent {
    ///How to solve part1 of the days puzzle.
    fn solve_part1(_path_to_file: &str) -> anyhow::Result<()> {
        Ok(())
    }
    ///How to solve part2 of the days puzzle.
    fn solve_part2(_path_to_file: &str) -> anyhow::Result<()> {
        Ok(())
    }
}

