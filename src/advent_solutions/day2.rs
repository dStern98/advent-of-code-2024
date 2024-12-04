
use std::cmp::Ordering;

use anyhow::Context;

use super::{read_input_file, SolveAdvent};
pub struct Day2;

#[derive(Debug, Clone)]
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

    fn level_delta_ok(next_num: &i64, current_num: &i64, last_seen_ordering: &mut Option<Ordering>) -> bool {
        //! Check if two levels obey the rules outlined in the problem.
        let delta = next_num - current_num;
        if delta.abs() > 3 || delta.abs() < 1 {
            //Delta cannot be gt 3 or lt 1.
            return false;
        }
        //Ordering cannot change.
        let current_ordering = current_num.cmp(next_num);
        match last_seen_ordering {
            None => {
                *last_seen_ordering = Some(current_ordering);
            }
            Some(previous_ordering) => {
                if previous_ordering != &current_ordering {
                    return false;
                }
            }
        }
        true
    }

    fn is_safe_with_replacement(&self) -> bool {
        //! Brute-force solution to Part2. Simply try all possible combinations,
        //! removing one index at a time.
        if self.is_safe() {
            return true;
        }
        for idx in 0..self.report.len() {
            let mut new_report = self.clone();
            new_report.report.remove(idx);
            if new_report.is_safe() {
                return true;
            }
        }
        false
    }

    fn is_safe(&self) -> bool {
        //! Determine if a report is safe.
        //! The two rules are that:
        //! 1. The levels are either all increasing or all decreasing
        //! 2. Any two adjacent levels differ by at least one and at most three.
        let mut last_seen_ordering = None;
        for (current_num, next_num) in self.report.iter().zip(self.report.iter().skip(1)) {
            if !ReactorReport::level_delta_ok(next_num, current_num, &mut last_seen_ordering) {
                return false;
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
            println!("There are a total of {safe_reports} safe reports.");
            Ok(())
        }
        fn solve_part2(path_to_file: &str) -> anyhow::Result<()> {
            let file_contents = read_input_file(path_to_file)?;
            let reports: Result<Vec<ReactorReport>, anyhow::Error> = file_contents.lines().map(ReactorReport::try_new).collect();
            let mut reports = reports?;
            let mut safe_reports = 0;
            for report in reports.iter_mut() {
                if report.is_safe_with_replacement() {
                    safe_reports += 1;
                    continue;
                }
            }
            println!("There are a total of {safe_reports} safe reports using problem damper.");
            Ok(())
        }
}