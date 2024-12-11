mod advent_solutions;
use crate::advent_solutions::day11::Day11;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day11::solve_part1("input.txt")?;
    Day11::solve_part2("test.txt")?;
    Ok(())
}
