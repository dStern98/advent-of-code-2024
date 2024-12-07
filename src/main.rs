mod advent_solutions;
use crate::advent_solutions::day7::Day7;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day7::solve_part1("input.txt")?;
    Day7::solve_part2("input.txt")?;
    Ok(())
}
