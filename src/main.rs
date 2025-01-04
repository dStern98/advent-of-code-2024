mod advent_solutions;
use crate::advent_solutions::day24::Day24;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day24::solve_part1("input.txt")?;
    Day24::solve_part2("test.txt")?;
    Ok(())
}
