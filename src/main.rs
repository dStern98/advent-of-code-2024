mod advent_solutions;
use crate::advent_solutions::day16::Day16;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day16::solve_part1("input.txt")?;
    Day16::solve_part2("input.txt")?;
    Ok(())
}
