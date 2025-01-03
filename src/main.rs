mod advent_solutions;
use crate::advent_solutions::day18::Day18;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day18::solve_part1("input.txt")?;
    Day18::solve_part2("input.txt")?;
    Ok(())
}
