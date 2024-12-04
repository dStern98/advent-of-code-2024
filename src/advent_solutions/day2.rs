
use anyhow::Context;

use super::{read_input_file, SolveAdvent};
pub struct Day2;

#[derive(Debug)]
pub struct ReactorReport {
    report: Vec<i64>,
}


impl ReactorReport {
    fn try_new(row: &str) -> anyhow::Result<Self> {
        let report: Result<Vec<i64>, _> = row.trim().split(' ').filter(|substr| !substr.is_empty()).map(|substr| substr.parse::<i64>()).collect();
        let report = report.context("Failed to parse row into report")?;
        Ok(Self {
            report
        })
    }

    fn is_safe(&self) -> bool {
        //! Determine if a report is safe.
        //! The two rules are that:
        //! 1. The levels are either all increasing or all decreasing
        //! 2. Any two adjacent levels differ by at least one and at most three.
        let mut last_seen_ordering = None;
        for (current_num, next_num) in self.report.iter().zip(self.report.iter().skip(1)) {
            let delta = next_num - current_num;
            if delta.abs() > 3 || delta.abs() < 1 {
                return false;
            }
            let current_ordering = current_num.cmp(next_num);
            match last_seen_ordering {
                None => {
                    last_seen_ordering = Some(current_ordering);
                    continue;
                }
                Some(previous_ordering) => {
                    if previous_ordering != current_ordering {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl  SolveAdvent for Day2 {
        fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
            let file_contents = read_input_file(path_to_file)?;
            let reports: Result<Vec<ReactorReport>, anyhow::Error> = file_contents.lines().map(ReactorReport::try_new).collect();
            let reports = reports?;
            let mut safe_reports = 0;
            for report in reports {
                if report.is_safe() {
                    safe_reports += 1;
                }
            }
            println!("There are a total of {safe_reports} safe reports");
            Ok(())
        }
        fn solve_part2(_path_to_file: &str) -> anyhow::Result<()> {
            Ok(())
        }
}