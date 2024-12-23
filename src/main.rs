mod advent_solutions;
use crate::advent_solutions::day19::Day19;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day19::solve_part1("input.txt")?;
    Day19::solve_part2("input.txt")?;
    Ok(())
}
