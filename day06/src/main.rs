use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Clone)]
#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn read_map(file_name: &String) -> Vec<Vec<char>> {
    return fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(String::from)
        .map(|s| s.chars().collect())
        .collect();
}

fn find_start_pos(map: &Vec<Vec<char>>, start_dir: &mut Direction) -> (usize, usize) {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == '^' {
                let pos = (row,col);
                *start_dir = Direction::Up;
                return pos;
            }
            if map[row][col] == 'v' {
                let pos = (row,col);
                *start_dir = Direction::Down;
                return pos;
            }
            if map[row][col] == '<' {
                let pos = (row,col);
                *start_dir = Direction::Left;
                return pos;
            }
            if map[row][col] == '>' {
                let pos = (row,col);
                *start_dir = Direction::Right;
                return pos;
            }
        }
    }
    println!("Failed to find start pos");
    return (0,0);
}

fn go_right(pos: &mut (usize,usize),
        visited_pos: &mut HashSet<(usize,usize)>,
        dir: &mut Direction,
        map: &Vec<Vec<char>>) -> bool {
    while pos.1 < map[pos.0].len() && map[pos.0][pos.1+1] != '#' {
        pos.1 += 1;
        visited_pos.insert(*pos);
    }
    if pos.1+1 >= map[pos.0].len() {
        return true;
    }
    *dir = Direction::Down;
    return false;
}

fn go_down(pos: &mut (usize,usize),
        visited_pos: &mut HashSet<(usize,usize)>,
        dir: &mut Direction,
        map: &Vec<Vec<char>>) -> bool {
    while (pos.0+1<map.len()) && (map[pos.0+1][pos.1] != '#') {
        pos.0 += 1;
        visited_pos.insert(*pos);
    }
    if pos.0+1 >= map.len() {
        return true;
    }
    *dir = Direction::Left;
    return false;
}

fn go_left(pos: &mut (usize,usize),
        visited_pos: &mut HashSet<(usize,usize)>,
        dir: &mut Direction,
        map: &Vec<Vec<char>>) -> bool {
    while (pos.1>0) && (map[pos.0][pos.1-1] != '#') {
        pos.1 -= 1;
        visited_pos.insert(*pos);
    }
    if pos.1 == 0 {
        return true;
    }
    *dir = Direction::Up;
    return false;
}

fn go_up(pos: &mut (usize,usize),
        visited_pos: &mut HashSet<(usize,usize)>,
        dir: &mut Direction,
        map: &Vec<Vec<char>>) -> bool {
    while (pos.0>0) && (map[pos.0-1][pos.1] != '#') {
        pos.0 -= 1;
        visited_pos.insert(*pos);
    }
    if pos.0 == 0 {
        return true;
    }
    *dir = Direction::Right;
    return false;
}

fn part1(orig_pos: &(usize,usize),
        orig_dir: Direction,
        map: &Vec<Vec<char>>,
        visited_pos: &mut HashSet<(usize,usize)>) {
    let mut pos = orig_pos.clone();
    let mut done = false;
    let mut dir = orig_dir.clone();
    while !done {
        match dir {
            Direction::Up => done = go_up(&mut pos, visited_pos, &mut dir, &map),
            Direction::Down => done = go_down(&mut pos, visited_pos, &mut dir, &map),
            Direction::Left => done = go_left(&mut pos, visited_pos, &mut dir, &map),
            Direction::Right => done = go_right(&mut pos, visited_pos, &mut dir, &map),
        }
    }
    println!("Part 1 num visited: {}", visited_pos.len());
}

fn check_loop(visited_dirs: &mut HashMap<(usize,usize),Direction>, pos: &(usize,usize), dir: &Direction) -> bool {
    if visited_dirs.contains_key(pos) {
        if visited_dirs.get(pos).unwrap() == dir {
            return true;
        }
    }
    visited_dirs.insert(*pos, dir.clone());
    return false;
}

fn go_right_check_loop(pos: &mut (usize,usize),
        visited_dirs: &mut HashMap<(usize,usize),Direction>,
        dir: &mut Direction,
        map: &Vec<Vec<char>>,
        detected_loop: &mut bool) -> bool {
    while (pos.1+1) < map[pos.0].len() && map[pos.0][pos.1+1] != '#' {
        pos.1 += 1;
        if check_loop(visited_dirs, pos, dir) {
            *detected_loop = true;
            return true;
        }
    }
    if pos.1+1 >= map[pos.0].len() {
        return true;
    }
    *dir = Direction::Down;
    return false;
}

fn go_down_check_loop(pos: &mut (usize,usize),
        visited_dirs: &mut HashMap<(usize,usize),Direction>,
        dir: &mut Direction,
        map: &Vec<Vec<char>>,
        detected_loop: &mut bool) -> bool {
    while (pos.0+1<map.len()) && (map[pos.0+1][pos.1] != '#') {
        pos.0 += 1;
        if check_loop(visited_dirs, pos, dir) {
            *detected_loop = true;
            return true;
        }
    }
    if pos.0+1 >= map.len() {
        return true;
    }
    *dir = Direction::Left;
    return false;
}

fn go_left_check_loop(pos: &mut (usize,usize),
        visited_dirs: &mut HashMap<(usize,usize),Direction>,
        dir: &mut Direction,
        map: &Vec<Vec<char>>,
        detected_loop: &mut bool) -> bool {
    while (pos.1>0) && (map[pos.0][pos.1-1] != '#') {
        pos.1 -= 1;
        if check_loop(visited_dirs, pos, dir) {
            *detected_loop = true;
            return true;
        }
    }
    if pos.1 == 0 {
        return true;
    }
    *dir = Direction::Up;
    return false;
}

fn go_up_check_loop(pos: &mut (usize,usize),
        visited_dirs: &mut HashMap<(usize,usize),Direction>,
        dir: &mut Direction,
        map: &Vec<Vec<char>>,
        detected_loop: &mut bool) -> bool {
    while (pos.0>0) && (map[pos.0-1][pos.1] != '#') {
        pos.0 -= 1;
        if check_loop(visited_dirs, pos, dir) {
            *detected_loop = true;
            return true;
        }
    }
    if pos.0 == 0 {
        return true;
    }
    *dir = Direction::Right;
    return false;
}

fn part2(visited_pos: &HashSet<(usize,usize)>,
    orig_pos: &(usize,usize),
    orig_map: &Vec<Vec<char>>,
    orig_dir: Direction)
{
    let mut loop_count: usize = 0;
    let mut pos;
    let mut dir;
    let mut visited_dirs: HashMap<(usize,usize),Direction> = HashMap::new();
    let mut map;
    
    for new_pos in visited_pos {
        if new_pos == orig_pos {
            continue;
        }
        
        map = orig_map.clone();
        pos = orig_pos.clone();
        dir = orig_dir.clone();
        
        visited_dirs.clear();
        visited_dirs.insert(pos, dir.clone());
        
        map[new_pos.0][new_pos.1] = '#';
        
        let mut done: bool = false;
        let mut detected_loop: bool = false;
        while !done {
            match dir {
                Direction::Up => done = go_up_check_loop(&mut pos, &mut visited_dirs, &mut dir, &map, &mut detected_loop),
                Direction::Down => done = go_down_check_loop(&mut pos, &mut visited_dirs, &mut dir, &map, &mut detected_loop),
                Direction::Left => done = go_left_check_loop(&mut pos, &mut visited_dirs, &mut dir, &map, &mut detected_loop),
                Direction::Right => done = go_right_check_loop(&mut pos, &mut visited_dirs, &mut dir, &map, &mut detected_loop),
            }
        }
        if detected_loop {
            loop_count += 1;
        }
    }
    println!("Part2 loop detect count: {}", loop_count);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    
    let map: Vec<Vec<char>> = read_map(&args[1]);
    let orig_map = map.clone();
    let mut orig_dir = Direction::Up;
    let orig_pos = find_start_pos(&map, &mut orig_dir);
    let mut visited_pos: HashSet<(usize,usize)> = HashSet::new();
    
    part1(&orig_pos, orig_dir.clone(), &map, &mut visited_pos);
    part2(&visited_pos, &orig_pos, &orig_map, orig_dir.clone());
}
