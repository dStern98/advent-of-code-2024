mod advent_solutions;
use crate::advent_solutions::day22::Day22;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day22::solve_part1("input.txt")?;
    Day22::solve_part2("test.txt")?;
    Ok(())
}
