mod advent_solutions;
use crate::advent_solutions::day21::Day21;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day21::solve_part1("input.txt")?;
    Day21::solve_part2("test.txt")?;
    Ok(())
}
