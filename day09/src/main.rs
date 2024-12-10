use std::fs;
use std::env;

fn read_input(file_name: &String) -> Vec<char> {
    return fs::read_to_string(file_name)
        .map(|s| s.chars().collect::<Vec<_>>())
        .expect("Failed to collect")
        .into_iter()
        .filter(|c| c.is_numeric())
        .collect();
}

fn gen_expanded_blocks(input_str: &Vec<char>) -> Vec<i32> {
    let mut idx: i32 = 0;
    let mut i: usize = 0;
    let mut res: Vec<i32> = Vec::new();
    while i < input_str.len() {
        let len: i32 = input_str[i].to_digit(10).unwrap() as i32;
        let mut free_space: i32 = 0;
        if (i+1) < input_str.len() {
            free_space = input_str[i+1].to_digit(10).unwrap() as i32;
        }

        for _j in 0..len {
            res.push(idx);
        }
        for _j in 0..free_space {
            res.push(-1);
        }

        i += 2;
        idx += 1;
    }
    return res;
}

fn calc_checksum(blocks: &Vec<i32>) -> u64 {
    let mut res: u64 = 0;
    for i in 0..blocks.len() {
        if blocks[i] > 0 {
            res += ((i as u64)*(blocks[i] as u64)) as u64;
        }
    }
    return res;
}

fn part1(expanded_blocks: &Vec<i32>) {
    let mut blocks: Vec<i32> = expanded_blocks.clone();
    let mut left_idx: i64 = 0;
    let mut right_idx: i64 = (blocks.len()-1) as i64;
    while blocks[left_idx as usize] != -1 && (left_idx as usize) < blocks.len() {
        left_idx += 1;
    }
    while blocks[right_idx as usize] == -1 && right_idx >= 0 {
        right_idx -= 1;
    }

    loop {
        blocks[left_idx as usize] = blocks[right_idx as usize];
        blocks[right_idx as usize] = -1;

        let mut idx: i64 = left_idx;
        while (idx as usize) < blocks.len()-1 {
            if blocks[idx as usize] == -1 && (blocks[(idx+1) as usize] >= 0) {
                break;
            }
            idx += 1;
        }
        if idx as usize >= blocks.len()-1 {
            break;
        }

        while blocks[left_idx as usize] != -1 && (left_idx as usize) < blocks.len() {
            left_idx += 1;
        }
        while blocks[right_idx as usize] == -1 && right_idx >= 0 {
            right_idx -= 1;
        }
    }

    println!("Part1 checksum: {}", calc_checksum(&blocks));
}

fn part2(expanded_blocks: &Vec<i32>) {
    let mut blocks: Vec<i32> = expanded_blocks.clone();
    let mut right_idx: i64 = (blocks.len()-1) as i64;
    let mut cur_block_val: i32 = blocks[right_idx as usize];
    let mut cur_block_len: i32 = 0;
    let mut start_block_idx: usize = right_idx as usize;
    while blocks[right_idx as usize] == cur_block_val {
        right_idx -= 1;
        cur_block_len += 1;
    }

    while cur_block_val > 0 {
        let mut left_idx: i64 = 0;
        while left_idx < (right_idx+1) {
            if blocks[left_idx as usize] == -1 {
                let mut free_len: usize = 0;
                let mut start_free_idx: usize = left_idx as usize;
                while (left_idx as usize) < blocks.len() && blocks[left_idx as usize] == -1 {
                    left_idx += 1;
                    free_len += 1;
                }
                if (free_len as i32) >= cur_block_len {
                    for _i in 0..cur_block_len {
                        blocks[start_free_idx] = cur_block_val;
                        blocks[start_block_idx] = -1;
                        start_free_idx += 1;
                        start_block_idx -= 1;
                    }
                    break;
                }
            }
            else {
                left_idx += 1;
            }
        }

        let mut found_next: bool = false;
        while !found_next {
            cur_block_val -= 1;
            right_idx = (blocks.len()-1) as i64;
            while right_idx >= 0 && blocks[right_idx as usize] != cur_block_val {
                right_idx -= 1;
            }
            if right_idx < 0 {
                continue;
            }
            found_next = true;
            cur_block_len = 0;
            start_block_idx = right_idx as usize;
            while right_idx >= 0 && blocks[right_idx as usize] == cur_block_val {
                right_idx -= 1;
                cur_block_len += 1;
            }
        }
    }
    println!("Part2 checksum: {}", calc_checksum(&blocks));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    
    let input_str: Vec<char> = read_input(&args[1]);
    let expanded_blocks: Vec<i32> = gen_expanded_blocks(&input_str);

    part1(&expanded_blocks);
    part2(&expanded_blocks);
}

