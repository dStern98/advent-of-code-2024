mod advent_solutions;
use crate::advent_solutions::day12::Day12;
use crate::advent_solutions::SolveAdvent;

fn main() -> anyhow::Result<()> {
    Day12::solve_part1("input.txt")?;
    Day12::solve_part2("input.txt")?;
    Ok(())
}
