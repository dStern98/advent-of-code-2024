mod advent_solutions;
use crate::advent_solutions::day3::Day3;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day3::solve_part1("input.txt")?;
    Day3::solve_part2("input.txt")?;
    Ok(())
}
