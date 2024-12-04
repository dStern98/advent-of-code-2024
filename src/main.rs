mod advent_solutions;
use crate::advent_solutions::day2::Day2;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day2::solve_part1("input.txt")?;
    Day2::solve_part2("input.txt")?;
    Ok(())
}
