use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

type PosList = Vec<(i32,i32)>;

fn read_pos_map(file_name: &String) -> Vec<Vec<char>> {
    return fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|s| s.chars().collect())
        .collect();
}

fn gen_pos_list(map: &Vec<Vec<char>>) -> HashMap<char,PosList> {
    let mut pos_map: HashMap<char,PosList> = HashMap::new();
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == '.' {
                continue;
            }
            let ant = map[row][col];
            if !pos_map.contains_key(&ant) {
                pos_map.insert(ant,vec![(row as i32, col as i32)]);
            }
            else {
                pos_map.get_mut(&ant).expect("Key Failed").push((row as i32, col as i32));
            }
        }
    }
    return pos_map;
}

fn part1(map: &Vec<Vec<char>>, pos_map: &HashMap<char,PosList>) {
    let mut antinodes: HashSet<(i32,i32)> = HashSet::new();
    for pos_pair in pos_map {
        let poslist: &PosList = &pos_pair.1;
        for i in 0..poslist.len()-1 {
            for j in i+1..poslist.len() {
                
                let d_row: i32 = poslist[j].0 - poslist[i].0;
                let d_col: i32 = poslist[j].1 - poslist[i].1;

                let mut cur_row: i32 = poslist[j].0 + d_row;
                let mut cur_col: i32 = poslist[j].1 + d_col;
                if cur_row >= 0 &&
                        cur_row < map.len().try_into().unwrap() &&
                        cur_col >= 0 &&
                        cur_col < map[0].len().try_into().unwrap() {
                    antinodes.insert((cur_row,cur_col));
                }

                cur_row = poslist[i].0 - d_row;
                cur_col = poslist[i].1 - d_col;
                if cur_row >= 0 &&
                        cur_row < map.len().try_into().unwrap() &&
                        cur_col >= 0 &&
                        cur_col < map[0].len().try_into().unwrap() {
                    antinodes.insert((cur_row,cur_col));
                }
            }
        }
    }
    println!("Part1 size: {}", antinodes.len());
}

fn part2(map: &Vec<Vec<char>>, pos_map: &HashMap<char,PosList>) {
    let mut antinodes: HashSet<(i32,i32)> = HashSet::new();
    for pos_pair in pos_map {
        let poslist: &PosList = &pos_pair.1;
        for i in 0..poslist.len()-1 {
            for j in i+1..poslist.len() {
                
                let d_row: i32 = poslist[j].0 - poslist[i].0;
                let d_col: i32 = poslist[j].1 - poslist[i].1;

                antinodes.insert(poslist[i]);
                antinodes.insert(poslist[j]);

                let mut cur_row: i32 = poslist[j].0 + d_row;
                let mut cur_col: i32 = poslist[j].1 + d_col;
                while cur_row >= 0 &&
                        cur_row < map.len().try_into().unwrap() &&
                        cur_col >= 0 &&
                        cur_col < map[0].len().try_into().unwrap() {
                    antinodes.insert((cur_row,cur_col));

                    cur_row += d_row;
                    cur_col += d_col;
                }

                cur_row = poslist[i].0 - d_row;
                cur_col = poslist[i].1 - d_col;
                while cur_row >= 0 &&
                        cur_row < map.len().try_into().unwrap() &&
                        cur_col >= 0 &&
                        cur_col < map[0].len().try_into().unwrap() {
                    antinodes.insert((cur_row,cur_col));

                    cur_row -= d_row;
                    cur_col -= d_col;
                }
            }
        }
    }
    println!("Part2 size: {}", antinodes.len());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let map: Vec<Vec<char>> = read_pos_map(&args[1]);
    let pos_map: HashMap<char,PosList> = gen_pos_list(&map);

    part1(&map, &pos_map);
    part2(&map, &pos_map);
}

