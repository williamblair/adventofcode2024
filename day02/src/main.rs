use std::env;
use std::fs;

fn read_reports(file_name: String, reports: &mut Vec<Vec<i32>>) {
    let lines: Vec<String> = fs::read_to_string(file_name.clone())
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    for line in lines {
        let report_line: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Parse Error"))
            .collect();
        reports.push(report_line);
    }
}

fn check_is_safe(report: &Vec<i32>) -> bool {
    let mut prev_diff: i32 = 0;
    for i in 1..report.len() {
        let diff = report[i] - report[i-1];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if diff > 0 && prev_diff < 0 {
            return false;
        }
        if diff < 0 && prev_diff > 0 {
            return false;
        }
        prev_diff = diff;
    }
    return true;
}

fn part1(reports: &Vec<Vec<i32>>) {
    let mut safe_count = 0;
    for report in reports {
        if check_is_safe(report) {
            safe_count += 1;
        }
    }
    println!("Safe Count: {}", safe_count);
}

fn check_is_safe_p2(report: &Vec<i32>) -> bool {
    let default_safe: bool = check_is_safe(&report);
    if default_safe {
        return true;
    }

    // Try to find a modified report that passes w/ 1 level removed
    for i in 0..report.len() {

        // Create new vector w/ removed val
        let mut tmp_vec: Vec<i32> = Vec::new();
        for j in 0..report.len() {
            if i != j {
                tmp_vec.push(report[j]);
            }
        }

        // See if the removed level causes report pass
        if check_is_safe(&tmp_vec) {
            return true;
        }
    }

    // no modified reports pass
    return false;
}

fn part2(reports: &Vec<Vec<i32>>) {
    let mut safe_count = 0;
    for report in reports {
        if check_is_safe_p2(report) {
            safe_count += 1;
        }
    }
    println!("Safe Count p2: {}", safe_count);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let mut reports: Vec<Vec<i32>> = Vec::new();
    read_reports(args[1].clone(), &mut reports);

    part1(&reports);
    part2(&reports);
}

