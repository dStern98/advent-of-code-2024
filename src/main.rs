mod advent_solutions;
use crate::advent_solutions::day5::Day5;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day5::solve_part1("input.txt")?;
    Day5::solve_part2("input.txt")?;
    Ok(())
}
