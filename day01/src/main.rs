use std::env;
use std::fs;
use std::collections::HashMap;

fn read_lists(file_name: String, list1: &mut Vec<i32>, list2: &mut Vec<i32>) {
    let contents: String = fs::read_to_string(file_name.clone())
        .expect("Failed to open file");
    let contents: Vec<i32> = contents
        .split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect();
    for i in 0..contents.len()/2 {
        list1.push(contents[i*2+0]);
        list2.push(contents[i*2+1]);
    }
}

fn part1(mut list1: Vec<i32>, mut list2: Vec<i32>) {
    list1.sort();
    list2.sort();

    let mut sum: i32 = 0;
    for i in 0..list1.len() {
        sum += (list1[i] - list2[i]).abs();
    }
    println!("Diff sum: {}", sum);
}

fn part2(list1: Vec<i32>, list2: Vec<i32>) {
    let mut count_map: HashMap<i32,i32> = HashMap::new();
    let mut total: i32 = 0;
    for val1 in list1 {
       if !count_map.contains_key(&val1) {
           let mut count: i32 = 0;
           for val2 in &list2 {
               if val1 == *val2 {
                   count += 1;
               }
           }
           count_map.insert(val1, count);
       }
       let count = count_map.get(&val1).unwrap();
       total += val1 * count;
    }
    println!("Total: {}", total);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    read_lists(args[1].clone(), &mut list1, &mut list2);
    
    part1(list1.clone(), list2.clone());
    part2(list1.clone(), list2.clone());
}

