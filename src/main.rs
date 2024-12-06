mod advent_solutions;
use crate::advent_solutions::day6::Day6;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day6::solve_part1("input.txt")?;
    Day6::solve_part2("input.txt")?;
    Ok(())
}
