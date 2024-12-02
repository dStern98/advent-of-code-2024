mod advent_solutions;
use crate::advent_solutions::day1::Day1;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day1::solve_part1("input.txt")?;
    Day1::solve_part2("input.txt")?;
    Ok(())
}
