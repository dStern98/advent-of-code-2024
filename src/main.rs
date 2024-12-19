mod advent_solutions;
use crate::advent_solutions::day17::Day17;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day17::solve_part1("input.txt")?;
    Day17::solve_part2("input.txt")?;
    Ok(())
}
