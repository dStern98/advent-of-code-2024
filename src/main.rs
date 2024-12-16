mod advent_solutions;
use crate::advent_solutions::day14::Day14;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day14::solve_part1("input.txt")?;
    Day14::solve_part2("test.txt")?;
    Ok(())
}
