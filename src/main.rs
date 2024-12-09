mod advent_solutions;
use crate::advent_solutions::day8::Day8;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day8::solve_part1("input.txt")?;
    Day8::solve_part2("input.txt")?;
    Ok(())
}
