mod advent_solutions;
use crate::advent_solutions::day25::Day25;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day25::solve_part1("input.txt")?;
    Day25::solve_part2("test.txt")?;
    Ok(())
}
