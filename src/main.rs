mod advent_solutions;
use crate::advent_solutions::day13::Day13;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day13::solve_part1("input.txt")?;
    Day13::solve_part2("test.txt")?;
    Ok(())
}
