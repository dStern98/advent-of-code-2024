mod advent_solutions;
use crate::advent_solutions::day9::Day9;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day9::solve_part1("input.txt")?;
    Day9::solve_part2("input.txt")?;
    Ok(())
}
