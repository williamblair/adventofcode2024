use std::env;
use std::fs;
use regex::Regex;

fn read_input(file_name: String) -> String {
    return fs::read_to_string(file_name.clone())
        .unwrap();
}

fn do_mul(m: &str) -> i32 {
    let parsed = sscanf::sscanf!(m, "mul({},{})", i32, i32);
    let vals = parsed.unwrap();
    return vals.0 * vals.1;
}

fn part1(input: &String) {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let sum: i32 = re.find_iter(input)
        .map(|m| do_mul(m.as_str()))
        .sum();
    println!("Part1 sum: {}", sum);
}

fn do_cond_mul(m: &str, should_mul: &mut bool) -> i32 {
    let mut product: i32 = 0;
    let s: Vec<_> = m.chars().collect();
    if s[0] == 'd' && s[1] == 'o' && s[2] == '(' {
        *should_mul = true;
    }
    else if s[0] == 'd' && s[1] == 'o' && s[2] == 'n' {
        *should_mul = false;
    }
    else if *should_mul {
        product = do_mul(m);
    }
    return product;
}

fn part2(input: &String) {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)").unwrap();
    let mut should_mul: bool = true;
    let sum: i32 = re.find_iter(input)
        .map(|m| do_cond_mul(m.as_str(), &mut should_mul))
        .sum();
    println!("Part2 sum: {}", sum);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let input: String = read_input(args[1].clone());

    part1(&input);
    part2(&input);
}

