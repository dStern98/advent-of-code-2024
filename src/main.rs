mod advent_solutions;
use crate::advent_solutions::day23::Day23;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day23::solve_part1("input.txt")?;
    Day23::solve_part2("test.txt")?;
    Ok(())
}
