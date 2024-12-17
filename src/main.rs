mod advent_solutions;
use crate::advent_solutions::day15::Day15;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day15::solve_part1("input.txt")?;
    Day15::solve_part2("input.txt")?;
    Ok(())
}
