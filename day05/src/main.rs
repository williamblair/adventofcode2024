use std::fs;
use std::env;
use std::collections::HashMap;
use std::cmp::Ordering;

fn read_rules_map(file_name: &String) -> HashMap<i32,Vec<i32>> {
    let lines: Vec<String> = fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let mut rules_map: HashMap<i32,Vec<i32>> = HashMap::new();
    for line in lines {
        let parsed = sscanf::sscanf!(line, "{}|{}", i32, i32);
        let vals = parsed.unwrap();
        if !rules_map.contains_key(&vals.0) {
            let mut new_vec: Vec<i32> = Vec::new();
            new_vec.push(vals.1);
            rules_map.insert(vals.0, new_vec);
        }
        else {
            rules_map.get_mut(&vals.0)
                .expect("No key found")
                .push(vals.1);
        }
    }
    return rules_map;
}

fn read_pages_input(file_name: &String) -> Vec<Vec<i32>> {
    let lines: Vec<String> = fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let mut result: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        result.push(
            line.split(',')
                .map(|s| s.parse().unwrap())
                .collect()
        );
    }
    return result;
}

fn pages_valid(pages: &Vec<i32>, rules_map: &HashMap<i32,Vec<i32>>) -> bool {
    for i in 0..pages.len() {
        let cur_val: i32 = pages[i];
        if !rules_map.contains_key(&cur_val) {
            continue;
        }
        let rule_pages: &Vec<i32> = rules_map.get(&cur_val)
            .expect("No key found");
        for j in 0..i {
            if rule_pages.contains(&pages[j]) {
                return false;
            }
        }
    }
    return true;
}

fn part1(rules_map: &HashMap<i32,Vec<i32>>, pages_input: &Vec<Vec<i32>>) {
    let mut middle_sum: i32 = 0;
    for pages_line in pages_input {
        if pages_valid(pages_line, rules_map) {
            let middle_idx: usize = pages_line.len()/2;
            let middle_val: i32 = pages_line[middle_idx];
            middle_sum += middle_val;
        }
    }
    println!("Part1 middle sum: {}", middle_sum);
}

fn part2(rules_map: &HashMap<i32,Vec<i32>>, pages_input: &mut Vec<Vec<i32>>) {
    let mut middle_sum: i32 = 0;
    for pages_line in pages_input {
        if pages_valid(pages_line, rules_map) {
            continue;
        }
        pages_line.sort_by(|a, b| {
            if rules_map.contains_key(&a) {
                let before_vals: &Vec<i32> = rules_map.get(&a)
                    .expect("Failed to find a");
                if before_vals.contains(&b) {
                    return Ordering::Less;
                }
            }
            if rules_map.contains_key(&b) {
                let after_vals: &Vec<i32> = rules_map.get(&b)
                    .expect("Failed to find b");
                if after_vals.contains(&a) {
                    return Ordering::Greater;
                }
            }
            return Ordering::Equal;
            });
        let middle_idx: usize = pages_line.len()/2;
        let middle_val: i32 = pages_line[middle_idx];
        middle_sum += middle_val;
    }
    println!("Part2 middle sum: {}", middle_sum);
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <rules file> <pages file>", args[0]);
        return;
    }
    
    let rules_map: HashMap<i32,Vec<i32>> = read_rules_map(&args[1]);
    let mut pages_input: Vec<Vec<i32>> = read_pages_input(&args[2]);
    
    part1(&rules_map, &pages_input);
    part2(&rules_map, &mut pages_input);
}
