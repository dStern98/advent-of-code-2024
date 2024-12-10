mod advent_solutions;
use crate::advent_solutions::day10::Day10;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day10::solve_part1("input.txt")?;
    Day10::solve_part2("input.txt")?;
    Ok(())
}
