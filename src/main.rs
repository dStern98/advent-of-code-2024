mod advent_solutions;
use crate::advent_solutions::day4::Day4;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day4::solve_part1("input.txt")?;
    Day4::solve_part2("input.txt")?;
    Ok(())
}
