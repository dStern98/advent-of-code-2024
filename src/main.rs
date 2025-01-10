mod advent_solutions;
use crate::advent_solutions::day20::Day20;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day20::solve_part1("input.txt")?;
    Day20::solve_part2("input.txt")?;
    Ok(())
}
