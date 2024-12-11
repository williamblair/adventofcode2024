use std::fs;
use std::env;
use std::collections::HashSet;

fn read_input_map(file_name: &String) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let lines: Vec<String> = fs::read_to_string(&file_name)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    for line in lines {
        res.push(line.chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect());
    }
    return res;
}

fn do_trail_search(
        in_map: &Vec<Vec<i32>>,
        trail_ends: &mut HashSet<(usize,usize)>,
        cur_row: usize,
        cur_col: usize,
        target_pos: i32) {

    // check right
    let mut new_row: usize = cur_row;
    let mut new_col: usize = cur_col+1;
    if new_col < in_map[0].len() && in_map[new_row][new_col] == target_pos {
        if target_pos == 9 {
            trail_ends.insert((new_row,new_col));
        }
        else {
            do_trail_search(in_map, trail_ends, new_row, new_col, target_pos+1);
        }
    }
    // check down
    new_row = cur_row+1;
    new_col = cur_col;
    if new_row < in_map.len() && in_map[new_row][new_col] == target_pos {
        if target_pos == 9 {
            trail_ends.insert((new_row,new_col));
        }
        else {
            do_trail_search(in_map, trail_ends, new_row, new_col, target_pos+1);
        }
    }
    // check left
    new_row = cur_row;
    if cur_col > 0 {
        new_col = cur_col-1;
    }
    if cur_col > 0 && in_map[new_row][new_col] == target_pos {
        if target_pos == 9 {
            trail_ends.insert((new_row,new_col));
        }
        else {
            do_trail_search(in_map, trail_ends, new_row, new_col, target_pos+1);
        }
    }
    // check up
    if cur_row > 0 {
        new_row = cur_row-1;
    }
    new_col = cur_col;
    if cur_row > 0 && in_map[new_row][new_col] == target_pos {
        if target_pos == 9 {
            trail_ends.insert((new_row,new_col));
        }
        else {
            do_trail_search(in_map, trail_ends, new_row, new_col, target_pos+1);
        }
    }
}
fn get_trail_count(in_map: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
    let mut trail_ends: HashSet<(usize,usize)> = HashSet::new();
    do_trail_search(in_map, &mut trail_ends, row, col, 1);
    return trail_ends.len() as i32;
}
fn part1(in_map: &Vec<Vec<i32>>){
    let mut trailhead_sum: i32 = 0;
    for row in 0..in_map.len() {
        for col in 0..in_map[0].len() {
            if in_map[row][col] == 0 {
                let count: i32 = get_trail_count(in_map, row, col);
                trailhead_sum += count;
            }
        }
    }
    println!("Part1 sum: {}", trailhead_sum);
}

fn do_trail_rating_search(
        in_map: &Vec<Vec<i32>>,
        trail_ends: &mut Vec<(usize,usize)>,
        cur_row: usize,
        cur_col: usize,
        target_pos: i32) {

    // check right
    let mut new_row: usize = cur_row;
    let mut new_col: usize = cur_col+1;
    if new_col < in_map[0].len() && in_map[new_row][new_col] == target_pos {
        if target_pos == 9 {
            trail_ends.push((new_row,new_col));
        }
        else {
            do_trail_rating_search(in_map, trail_ends, new_row, new_col, target_pos+1);
        }
    }
    // check down
    new_row = cur_row+1;
    new_col = cur_col;
    if new_row < in_map.len() && in_map[new_row][new_col] == target_pos {
        if target_pos == 9 {
            trail_ends.push((new_row,new_col));
        }
        else {
            do_trail_rating_search(in_map, trail_ends, new_row, new_col, target_pos+1);
        }
    }
    // check left
    new_row = cur_row;
    if cur_col > 0 {
        new_col = cur_col-1;
    }
    if cur_col > 0 && in_map[new_row][new_col] == target_pos {
        if target_pos == 9 {
            trail_ends.push((new_row,new_col));
        }
        else {
            do_trail_rating_search(in_map, trail_ends, new_row, new_col, target_pos+1);
        }
    }
    // check up
    if cur_row > 0 {
        new_row = cur_row-1;
    }
    new_col = cur_col;
    if cur_row > 0 && in_map[new_row][new_col] == target_pos {
        if target_pos == 9 {
            trail_ends.push((new_row,new_col));
        }
        else {
            do_trail_rating_search(in_map, trail_ends, new_row, new_col, target_pos+1);
        }
    }
}
fn get_trail_rating_count(in_map: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
    let mut trail_ends: Vec<(usize,usize)> = Vec::new();
    do_trail_rating_search(in_map, &mut trail_ends, row, col, 1);
    return trail_ends.len() as i32;
}
fn part2(in_map: &Vec<Vec<i32>>){
    let mut trailhead_sum: i32 = 0;
    for row in 0..in_map.len() {
        for col in 0..in_map[0].len() {
            if in_map[row][col] == 0 {
                let count: i32 = get_trail_rating_count(in_map, row, col);
                trailhead_sum += count;
            }
        }
    }
    println!("Part2 sum: {}", trailhead_sum);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", &args[0]);
        return;
    }
    let in_map: Vec<Vec<i32>> = read_input_map(&args[1]);

    part1(&in_map);
    part2(&in_map);
}

